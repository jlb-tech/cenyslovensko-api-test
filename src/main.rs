use cenyslovensko_product_api::{
    adapters::out::http::product_price::HttpProductPriceSource,
    application::get_current_day_product_prices_use_case::GetCurrentDayProductPricesUseCase,
    domain::product_price::{ProductPricesCurrentDayQuery, SortOrder},
};
use cenyslovensko_web_client::WebClientConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let web_client = WebClientConfig::new("https://api.cenyslovensko.sk/").build()?;
    let source = HttpProductPriceSource::new(web_client, "api/product-prices/current-day");
    let use_case = GetCurrentDayProductPricesUseCase::new(source);
    let staples = [
        "mlieko", "vajc", "ryz", "ryža", "muka", "múka", "kur", "cest", "zemiak", "paradaj",
        "maslo", "olej", "cukor", "syr", "smot", "jogurt", "chlieb",
    ];
    for page_number in 0..1 {
        let query = ProductPricesCurrentDayQuery::builder()
            .branch_ids(["1061_50020188", "1015_50020188"])
            .order_by("unit_price")
            .sort_order(SortOrder::Asc)
            .only_in_my_branches(true)
            .page(page_number)
            .size(10)
            .build();

        let page = use_case.execute(query).await?;
        println!(
            "page={} count={} returned={}",
            page.page,
            page.count,
            page.content.len()
        );
        for product in page.content {
            let name = product.product_details.product_name.to_lowercase();
            if !staples.iter().any(|term| name.contains(term)) {
                continue;
            }
            let prices = product
                .prices
                .iter()
                .map(|price| format!("{}={:.2}", price.branch_id, price.price.unwrap_or_default()))
                .collect::<Vec<_>>()
                .join(",");
            let escaped_name: String = product
                .product_details
                .product_name
                .chars()
                .map(|character| {
                    if character.is_ascii() {
                        character.to_string()
                    } else {
                        format!("\\u{{{:x}}}", character as u32)
                    }
                })
                .collect();
            println!(
                "name={} ean={} report_date={} prices=[{}]",
                escaped_name, product.ean, product.report_date, prices
            );
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
