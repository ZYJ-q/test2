use std::collections::VecDeque;
use std::{collections::HashMap, fs, time::Duration};

use chrono::Utc;
use log::{debug, info, warn};
use serde_json::{Map, Value};
// use tokio::{sync::broadcast::{self, Receiver}};
use test_alarm::adapters::binance::futures::http::actions::BinanceFuturesApi;
use test_alarm::actors::*;
// use test_alarm::models::http_data::*;

#[warn(unused_mut, unused_variables, dead_code)]
async fn real_time(
    futures: &Value,
    ori_fund: f64,
) {
    //rece: &mut Receiver<&str>){
    info!("get ready for real time loop");
    // let mut running = false;

    // let mut i = 0;

    // 每个品种的上一个trade_id

    // 净值数据
    // let mut net_worth_histories: VecDeque<Value> = VecDeque::new();

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
       
        // 时间
        // map.insert(String::from("time"), Value::from(date));

        // 账户余额
        info!("account balance");

        // let v = serde_json::from_value(&futures).unwrap();

        println!("futures数据{:?}", futures);
    // let mut net_worth: f64 = f64::INFINITY;
    // let mut new_account_object: Map<String, Value> = Map::new();
    // let mut account_object: Map<String, Value> = Map::new();
    // if let Some(data) = binance_futures_api.account(None).await {
    //     let v: Value = serde_json::from_str(&data).unwrap();
    //     let wallet_total: f64 = v
    //         .as_object()
    //         .unwrap()
    //         .get("totalWalletBalance")
    //         .unwrap()
    //         .as_str()
    //         .unwrap()
    //         .parse()
    //         .unwrap();
    //     let pnl_total: f64 = v
    //         .as_object()
    //         .unwrap()
    //         .get("totalUnrealizedProfit")
    //         .unwrap()
    //         .as_str()
    //         .unwrap()
    //         .parse()
    //         .unwrap();
    //     let notional_total = wallet_total + pnl_total; // 权益 = 余额 + 未实现盈亏
    //     let leverage_total = wallet_total / notional_total; // 杠杆率 = 余额 / 权益
    //     let total_equity = wallet_total + pnl_total;
    //     let margin_balance: f64 = v
    //         .as_object()
    //         .unwrap()
    //         .get("totalMarginBalance")
    //         .unwrap()
    //         .as_str()
    //         .unwrap()
    //         .parse()
    //         .unwrap();
    //     new_account_object.insert(
    //         String::from("total_equity"),
    //         Value::from(total_equity.to_string()),
    //     );
    //     new_account_object.insert(
    //         String::from("time"), 
    //         Value::from(date),
    //     );
    //     account_object.insert(
    //         String::from("wallet"),
    //         Value::from(wallet_total.to_string()),
    //     );
    //     account_object.insert(
    //         String::from("notional"),
    //         Value::from(notional_total.to_string()),
    //     );
    //     account_object.insert(
    //         String::from("leverage"),
    //         Value::from(leverage_total.to_string()),
    //     );
    //     account_object.insert(
    //         String::from("margin"),
    //         Value::from(margin_balance.to_string()),
    //     );
    //     net_worth = notional_total/ori_fund;
    //     net_worth_histories.push_back(Value::from(new_account_object));
    // }
    // map.insert(String::from("account"), Value::from(account_object));

        

        // 成交历史(更新所有)
        

        // 仓位

        // 净值
        // map.insert(String::from("net_worth"), Value::from(net_worth.to_string()));

        // 装入文件
        info!("write into json");
        // json_data.insert(String::from("type"), Value::from(String::from("interval")));
        // json_data.insert(String::from("data"), Value::from(map));
        // response.insert(String::from("response"), Value::from(json_data));
        // let json_file = Value::from(response).to_string();
        // let mut path = std::env::current_dir().unwrap();
        // path.pop();
        // path.pop();

        // 输出日志
        // debug!("writing {}", json_file);

        // 输出文件
        info!("write into file");
        // fs::write(
        //     format!("{}/output/out.json", path.to_str().unwrap()),
        //     String::from(&json_file),
        // )
        // .unwrap();

        // if trade_histories

        // if ssh_api.get_root_name() == "nnn2" {
        //     println!("lzq账户{:?}", Vec::from(trade_histories.clone())); 
        //     println!("lzq账户{:?}", Vec::from(positions.clone())); 
        //     println!("lzq账户{:?}", Vec::from(net_worth_histories.clone())); 

        // }

        // let net_worth_res = trade_mapper::NetWorkMapper::insert_net_worth(Vec::from(net_worth_histories.clone()));
        // print!("输出的净值数据信息{}", net_worth_res);

        // 等待下次执行
        info!("waiting for next real time task...({})", 90000 * 10);
        tokio::time::delay_for(Duration::from_millis(90000 * 10)).await;
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
        // let server_config = config.get("Server").unwrap();
        let futures = binance_config.get("futures").unwrap();
        let key = config.get("Alarm").unwrap().get("webhook").unwrap().as_str().unwrap();
        // info!("获取key");
        let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
        wxbot.push_str(key);
        info!("wxbot  {}", wxbot);
        // let wx_robot = WxbotHttpClient::new(&wxbot);
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
        
        
        
        // let ssh_api = SshClient::new(
        //     server_config.get("host").unwrap().as_str().unwrap(),
        //     server_config.get("port").unwrap().as_str().unwrap(),
        //     server_config.get("username").unwrap().as_str().unwrap(),
        //     server_config.get("password").unwrap().as_str().unwrap(),
        //     server_config.get("root_path").unwrap().as_str().unwrap(),
        //     server_config.get("root_name").unwrap().as_str().unwrap(),
        // );
        

        
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
        // let binance_futures_api=BinanceFuturesApi::new(
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("base_url")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("api_key")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("secret_key")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        // );

        
        info!("created http client");
        real_time(futures, 500.0).await;
    });

    // 开始任务
    info!("alarm begin(binance account)");
    real_time_handle.await.unwrap();
    info!("alarm done");
}
