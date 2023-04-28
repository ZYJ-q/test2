use std::collections::VecDeque;
use std::{collections::HashMap, fs, time::Duration};

use chrono::{DateTime, NaiveDateTime, Utc};
use log::{debug, info, warn};
use serde_json::{Map, Value};
// use tokio::{sync::broadcast::{self, Receiver}};
use test_alarm::adapters::binance::futures::http::actions::BinanceFuturesApi;
use test_alarm::base::ssh::SshClient;
use test_alarm::base::wxbot::WxbotHttpClient;
use test_alarm::actors::*;
// use test_alarm::models::http_data::*;

#[warn(unused_mut, unused_variables, dead_code)]
async fn real_time(
    binance_futures_api: BinanceFuturesApi,
    symbols: &Vec<Value>,
    mut ssh_api: SshClient,
    wx_robot: WxbotHttpClient,
    ori_fund: f64,
) {
    //rece: &mut Receiver<&str>){
    info!("get ready for real time loop");
    let mut running = false;

    let mut i = 0;

    // 每个品种的上一个trade_id
    let mut last_trade_ids: HashMap<String, u64> = HashMap::new();
    for symbol_v in symbols {
        let symbol = String::from(symbol_v.as_str().unwrap());
        let symbol = format!("{}USDT", symbol);
        last_trade_ids.insert(symbol, 0);
    }

    // 交易历史
    let mut trade_histories: VecDeque<Value> = VecDeque::new();

    // 净值数据
    let mut net_worth_histories: VecDeque<Value> = VecDeque::new();

    info!("begin real time loop");
    // 监控循环
    loop {
        info!("again");
        // json对象
        let mut response: Map<String, Value> = Map::new();
        let mut json_data: Map<String, Value> = Map::new();
        let mut map: Map<String, Value> = Map::new();
        map.insert(String::from("productId"), Value::from("TRADER_001"));
        let now = Utc::now();
        let date = format!("{}", now.format("%Y/%m/%d %H:%M:%S"));

        // 监控服务器状态
        info!("server process");
        // let mut server_status: VecDeque<Value> = VecDeque::new();
        let mut server_process: Map<String, Value> = Map::new();
        print!("判断是true还是false {}", ssh_api.search_py_ps());
        match ssh_api.search_py_ps() {
            true => {
                if !running {
                    running = true;
                    print!("改变running的值{}", running);
                    let sender = "程序开启";
                    let content = format!("process name: {}", ssh_api.get_root_name());
                    wx_robot.send_text(sender, &content).await;
                }
                server_process.insert(String::from("status"), Value::from("running"));
                server_process.insert(String::from("info"), Value::from(""));
            }
            false => {
                server_process.insert(String::from("status"), Value::from("stopped"));
                let mut info = ssh_api.download_log();
                if running {
                    running = false;
                    let sender = "程序停止";
                    let content;
                    if info == "" {
                        content = format!("{}: 未找到错误，请查看日志", ssh_api.get_root_name());
                    }else {
                        content = format!("{}: {}", ssh_api.get_root_name(), &info);
                    }
                    wx_robot.send_text(sender, &content).await;
                    info = content;
                }
                server_process.insert(String::from("info"), Value::from(info));
            }
        }
        map.insert(String::from("server"), Value::from(server_process));


        print!("running的值是否被改变{}", running);

        // 时间
        // map.insert(String::from("time"), Value::from(date));

        // 账户余额
        info!("account balance");
    let mut net_worth: f64 = f64::INFINITY;
    let mut new_account_object: Map<String, Value> = Map::new();
    let mut account_object: Map<String, Value> = Map::new();
    if let Some(data) = binance_futures_api.account(None).await {
        let v: Value = serde_json::from_str(&data).unwrap();
        let wallet_total: f64 = v
            .as_object()
            .unwrap()
            .get("totalWalletBalance")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        let pnl_total: f64 = v
            .as_object()
            .unwrap()
            .get("totalUnrealizedProfit")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        let notional_total = wallet_total + pnl_total; // 权益 = 余额 + 未实现盈亏
        let leverage_total = wallet_total / notional_total; // 杠杆率 = 余额 / 权益
        let total_equity = wallet_total + pnl_total;
        let margin_balance: f64 = v
            .as_object()
            .unwrap()
            .get("totalMarginBalance")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        new_account_object.insert(
            String::from("total_equity"),
            Value::from(total_equity.to_string()),
        );
        new_account_object.insert(
            String::from("time"), 
            Value::from(date),
        );
        account_object.insert(
            String::from("wallet"),
            Value::from(wallet_total.to_string()),
        );
        account_object.insert(
            String::from("notional"),
            Value::from(notional_total.to_string()),
        );
        account_object.insert(
            String::from("leverage"),
            Value::from(leverage_total.to_string()),
        );
        account_object.insert(
            String::from("margin"),
            Value::from(margin_balance.to_string()),
        );
        net_worth = notional_total/ori_fund;
        net_worth_histories.push_back(Value::from(new_account_object));
    }
    map.insert(String::from("account"), Value::from(account_object));

        

        // 成交历史(更新所有)
        info!("trade history");
        for symbol_v in symbols {
            let mut trade_object: Map<String, Value> = Map::new();
            let symbol = symbol_v.as_str().unwrap();
            let symbol = format!("{}USDT", symbol);
            let last_trade_id = last_trade_ids.get(&symbol).unwrap().clone();
            info!("{:?}: {}",(&symbol), last_trade_id);
            if let Some(data) = binance_futures_api.trade_hiostory(&symbol).await {
                let v: Value = serde_json::from_str(&data).unwrap();
                match v.as_array() {
                    Some(value) => {
                        if value.len() == 0 {
                            continue;
                        } else {
                            info!("i的值: {}", i);
                            if i == value.len() {
                                continue;
                            }
                            let this_trade_id = value[i]
                                .as_object()
                                .unwrap()
                                .get("orderId")
                                .unwrap()
                                .as_u64()
                                .unwrap();
                            // info!("this_trade_id: {}", this_trade_id);
                            if last_trade_id != this_trade_id {
                                info!("this_trade_id: {} ,{}", this_trade_id, last_trade_id);
                                last_trade_ids.insert(String::from(&symbol), this_trade_id);
                                trade_object.insert(String::from("tra_symbol"), Value::from(symbol));
                                trade_object.insert(
                                    String::from("th_id"),
                                    Value::from(
                                        value[i]
                                            .as_object()
                                            .unwrap()
                                            .get("id")
                                            .unwrap()
                                            .as_u64()
                                            .unwrap(),
                                    ),
                                );
                                trade_object
                                    .insert(String::from("tra_order_id"), Value::from(this_trade_id));
                                // trade_object
                                //     .insert(String::from("tra_id"), Value::from(1));
                                trade_object.insert(
                                    String::from("side"),
                                    Value::from(
                                        value[i]
                                            .as_object()
                                            .unwrap()
                                            .get("side")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("price"),
                                    Value::from(
                                        value[i]
                                            .as_object()
                                            .unwrap()
                                            .get("price")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("qty"),
                                    Value::from(
                                        value[i]
                                            .as_object()
                                            .unwrap()
                                            .get("qty")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("realized_pnl"),
                                    Value::from(
                                        value[i]
                                            .as_object()
                                            .unwrap()
                                            .get("realizedPnl")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("quote_qty"),
                                    Value::from(
                                        value[i]
                                            .as_object()
                                            .unwrap()
                                            .get("quoteQty")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("position_side"),
                                    Value::from(
                                        value[i]
                                            .as_object()
                                            .unwrap()
                                            .get("positionSide")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("tra_commision"),
                                    Value::from(
                                        value[i]
                                            .as_object()
                                            .unwrap()
                                            .get("commission")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                let millis = value[i]
                                    .as_object()
                                    .unwrap()
                                    .get("time")
                                    .unwrap()
                                    .as_i64()
                                    .unwrap();
                                let datetime: DateTime<Utc> = DateTime::from_utc(
                                    NaiveDateTime::from_timestamp_millis(millis).unwrap(),
                                    Utc,
                                );
                                // info!("datetime: {}", datetime);
                                let time = format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));
                                trade_object.insert(String::from("tra_time"), Value::from(time.clone()));
                                // match value[i].as_object().unwrap().get("buyer") {
                                //     Some(buyer) => {
                                //         trade_object.insert(
                                //             String::from("is_buyer"),
                                //             Value::Bool(buyer.as_bool().unwrap()),
                                //         );
                                //     }
                                //     None => {
                                //         trade_object.insert(String::from("is_buyer"), Value::Null);
                                //     }
                                // }
                                match value[i].as_object().unwrap().get("maker") {
                                    Some(maker) => {
                                        trade_object.insert(
                                            String::from("is_maker"),
                                            Value::Bool(maker.as_bool().unwrap()),
                                        );
                                    }
                                    None => {
                                        trade_object.insert(String::from("is_maker"), Value::Null);
                                    }
                                }
                                info!("running 的值: {}", running);
                                if running {
                                    let sender = "订单成交";
                                    let strs: Vec<&str> = trade_object.get("tra_symbol").unwrap().as_str().unwrap().split(symbol_v.as_str().unwrap()).collect();
                                    let mut content = String::from(&time);
                                    content.push_str("");
                                    content.push_str(trade_object.get("side").unwrap().as_str().unwrap());
                                    content.push_str("");
                                    content.push_str(trade_object.get("qty").unwrap().as_str().unwrap());
                                    content.push_str(&format!("{} for ", symbol_v.as_str().unwrap()));
                                    content.push_str(trade_object.get("price").unwrap().as_str().unwrap());
                                    content.push_str(&format!("{} each", strs[1]));
                                    wx_robot.send_text(sender, &content).await;
                                    i += 1
                                }
                                trade_histories.push_back(Value::from(trade_object));
                                if trade_histories.len() > 1000 {
                                    trade_histories.pop_front();
                                }
                            // 如果last_trade_id = orderId
                            } else {
                                continue;
                            }
                        }
                    }
                    None => {
                        continue;
                    }
                }
            }
        }
        map.insert(
            String::from("trades"),
            Value::from(Vec::from(trade_histories.clone())),
        );
        

        // 仓位
        info!("positions");
        let mut positions: VecDeque<Value> = VecDeque::new();
        for symbol_v in symbols {
            let symbol = format!("{}USDT", symbol_v.as_str().unwrap());
            if let Some(data) = binance_futures_api.position(&symbol).await {
                let tra_id = 1;
                let value: Value = serde_json::from_str(&data).unwrap();
                let vec = value.as_array().unwrap();
                for v in vec {
                    let mut pos_obj: Map<String, Value> = Map::new();
                    let obj = v.as_object().unwrap(); // positionAmt positionSide
                    let update_time = obj.get("updateTime").unwrap().as_i64().unwrap();
                    if  update_time != 0 {
                        let symbol = obj.get("symbol").unwrap().as_str().unwrap();
                        let entry_price = obj.get("entryPrice").unwrap().as_str().unwrap();
                        let un_realized_profit = obj.get("unRealizedProfit").unwrap().as_str().unwrap();
                        let leverage = obj.get("leverage").unwrap().as_str().unwrap();
                        let mark_price = obj.get("markPrice").unwrap().as_str().unwrap();
                    
                        print!("update_time{}", update_time);
                        let position_amt = obj.get("positionAmt").unwrap().as_str().unwrap();
                        let amt: f64 = position_amt.parse().unwrap();
                        let position_side: &str;
                        if amt - 0.0 <= 1e-6 {
                           position_side = "flat";
                        } else if amt.is_sign_negative() {
                           position_side = "short";
                        } else {
                           position_side = "long";
                        }
                        let datetime: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp_millis(update_time).unwrap(), Utc,);
                        let time = format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));
                        pos_obj.insert(String::from("symbol"), Value::from(symbol));
                        pos_obj.insert(String::from("position_amt"), Value::from(position_amt));
                        pos_obj.insert(String::from("position_side"), Value::from(position_side));
                        pos_obj.insert(String::from("time"), Value::from(time.clone()));
                        pos_obj.insert(String::from("entry_price"), Value::from(entry_price));
                        pos_obj.insert(String::from("un_realized_profit"), Value::from(un_realized_profit));
                        pos_obj.insert(String::from("tra_id"), Value::from(tra_id));
                        pos_obj.insert(String::from("leverage"), Value::from(leverage));
                        pos_obj.insert(String::from("mark_price"), Value::from(mark_price));
                        positions.push_back(Value::from(pos_obj));
                    } else {
                        continue;
                    }
                    
                }
            }
        }
        map.insert(String::from("positions"), Value::from(Vec::from(positions.clone())));

        // 净值
        map.insert(String::from("net_worth"), Value::from(net_worth.to_string()));

        // 装入文件
        info!("write into json");
        json_data.insert(String::from("type"), Value::from(String::from("interval")));
        json_data.insert(String::from("data"), Value::from(map));
        response.insert(String::from("response"), Value::from(json_data));
        let json_file = Value::from(response).to_string();
        let mut path = std::env::current_dir().unwrap();
        path.pop();
        path.pop();

        // 输出日志
        debug!("writing {}", json_file);

        // 输出文件
        info!("write into file");
        fs::write(
            format!("{}/output/out.json", path.to_str().unwrap()),
            String::from(&json_file),
        )
        .unwrap();

        // if trade_histories

        // if ssh_api.get_root_name() == "nnn2" {
        //     println!("lzq账户{:?}", Vec::from(trade_histories.clone())); 
        //     println!("lzq账户{:?}", Vec::from(positions.clone())); 
        //     println!("lzq账户{:?}", Vec::from(net_worth_histories.clone())); 

        // }

        let res = trade_mapper::TradeMapper::insert_trade(Vec::from(trade_histories.clone()));
        println!("插入历史交易数据是否成功{}", res);
        
        let po_res = trade_mapper::PositionMapper::insert_position(Vec::from(positions.clone()));
        print!("输出的仓位数据信息{}", po_res);

        let net_worth_res = trade_mapper::NetWorkMapper::insert_net_worth(Vec::from(net_worth_histories.clone()));
        print!("输出的净值数据信息{}", net_worth_res);

        // 等待下次执行
        info!("waiting for next real time task...({})", 1000 * 10);
        tokio::time::delay_for(Duration::from_millis(1000 * 10)).await;
    }
}

#[warn(unused_mut, unused_variables)]
#[tokio::main]
async fn main() {
    // 日志
    log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();

    init();

    // 测试用api
    // let api_key="JwYo1CffkOLqmv2sC3Qhe2Qu5GgzbeLVw2BxWB5HgK6tnmc8yGfkzLuDImBgDkXm";
    // let api_secret="7FtQARZqM2PDgIZ5plr3nwEVYBXXbvmSuvmpf6Viz9e7Cq2B87grRTG3VZQiEC5C";

    // 连接数据库
    // let config_db: Value =
    //     serde_json::from_str(&fs::read_to_string("./configs/database.json").unwrap()).unwrap();

    // 读取配置
    let config: Value = serde_json::from_str(
        &fs::read_to_string("./configs/total.json").expect("Unable to read file"),
    )
    .expect("Unable to parse");

    // 任务间通信信道
    // let (send, mut rece) = broadcast::channel(32);

    // 创建任务
    let real_time_handle = tokio::spawn(async move {
        // let mut futures_config: Map<String, Value> = Map::new();
        // let mut servers_config = Map::new();
        let binance_config = config.get("Binance").unwrap();
        // let binance_future_config = binance_config.get("futures").unwrap();
        let server_config = config.get("Server").unwrap();
        let symbols = config.get("Symbols").unwrap().as_array().unwrap();
        let key = config.get("Alarm").unwrap().get("webhook").unwrap().as_str().unwrap();
        // info!("获取key");
        let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
        wxbot.push_str(key);
        info!("wxbot  {}", wxbot);
        let wx_robot = WxbotHttpClient::new(&wxbot);
        info!("preparing...");

        // for s_config in server_config{
        //     let obj = s_config.as_object().unwrap(); 
        //     let host = obj.get("host").unwrap().as_str().unwrap();
        //     let port = obj.get("port").unwrap().as_str().unwrap();
        //     let username = obj.get("username").unwrap().as_str().unwrap();
        //     let password = obj.get("password").unwrap().as_str().unwrap();
        //     let root_path = obj.get("root_path").unwrap().as_str().unwrap();
        //     let root_name = obj.get("root_name").unwrap().as_str().unwrap();
        //     servers_config.insert(String::from("host"), Value::from(host));
        //     servers_config.insert(String::from("port"), Value::from(port));
        //     servers_config.insert(String::from("username"), Value::from(username));
        //     servers_config.insert(String::from("password"), Value::from(password));
        //     servers_config.insert(String::from("root_path"), Value::from(root_path));
        //     servers_config.insert(String::from("root_name"), Value::from(root_name));
        // }
        
        
        
        let ssh_api = SshClient::new(
            server_config.get("host").unwrap().as_str().unwrap(),
            server_config.get("port").unwrap().as_str().unwrap(),
            server_config.get("username").unwrap().as_str().unwrap(),
            server_config.get("password").unwrap().as_str().unwrap(),
            server_config.get("root_path").unwrap().as_str().unwrap(),
            server_config.get("root_name").unwrap().as_str().unwrap(),
        );
        

        
        // for f_config in binance_future_config{
        //     let obj = f_config.as_object().unwrap(); 
        //     let base_url = obj.get("base_url").unwrap().as_str().unwrap();
        //     let api_key = obj.get("api_key").unwrap().as_str().unwrap();
        //     let secret_key = obj.get("secret_key").unwrap().as_str().unwrap();
        //     futures_config.insert(String::from("base_url"), Value::from(base_url));
        //     futures_config.insert(String::from("api_key"), Value::from(api_key));
        //     futures_config.insert(String::from("secret_key"), Value::from(secret_key));
        // }

        info!("created ssh client");
        let binance_futures_api=BinanceFuturesApi::new(
            binance_config
                .get("futures")
                .unwrap()
                .get("base_url")
                .unwrap()
                .as_str()
                .unwrap(),
            binance_config
                .get("futures")
                .unwrap()
                .get("api_key")
                .unwrap()
                .as_str()
                .unwrap(),
            binance_config
                .get("futures")
                .unwrap()
                .get("secret_key")
                .unwrap()
                .as_str()
                .unwrap(),
        );

        
        info!("created http client");
        real_time(binance_futures_api, symbols, ssh_api, wx_robot, 500.0).await;
    });

    // 开始任务
    info!("alarm begin(binance account)");
    real_time_handle.await.unwrap();
    info!("alarm done");
}
