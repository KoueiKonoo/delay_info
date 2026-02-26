use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;
use dotenv::dotenv;
use std::env;

type MyResult<T> = Result<T, Error>;

// APIから返ってくるJSONの構造を定義
#[derive(Deserialize, Debug)]
struct TrainInfo {
    #[serde(rename = "odpt:railway")]
    railway: String,
    #[serde(rename = "odpt:trainInformationText")]
    text: Option<HashMap<String, String>>,
}

#[tokio::main]
pub async fn run() -> MyResult<()> {
    dotenv().ok();

    let token = env::var("ODPT_TOKEN").expect("ODPT_API_TOKEN must be set");
    let url = format!(
        "https://api.odpt.org/api/v4/odpt:TrainInformation?acl:consumerKey={}",
        token
    );

    // APIへリクエストを送信
    let response = reqwest::get(&url).await?;

    // 平常時などデータがない場合は空配列が返るため Vec で受け取る
    let train_infos: Vec<TrainInfo> = response.json().await?;

    // 抽出したい路線のキーワード（部分一致で検索）
    let target_keywords = ["Rinkai"];
    let mut has_delay_info = false;

    for info in train_infos {
        // 対象路線のいずれかが含まれているかチェック
        if target_keywords.iter().any(|&k| info.railway.contains(k)) {
            has_delay_info = true;

            // 日本語のテキスト情報を取得（存在しない場合のフォールバックも用意）
            let ja_text = info.text
                .as_ref()
                .and_then(|t| t.get("ja"))
                .map(|s| s.as_str())
                .unwrap_or("情報なし");

            println!("【{}】\n{}\n", info.railway, ja_text);
        }
    }

    if !has_delay_info {
        println!("現在、りんかい線に関する特別な運行情報（遅延・運休など）はありません。");
    }

    Ok(())
}