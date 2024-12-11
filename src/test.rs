#[cfg(test)]
mod tests {
    use std::env;

    use dotenv::dotenv;

    use crate::client::CalDAVClient;

    #[tokio::test]
    async fn test() {
        dotenv().ok();
        let base_url = env::var("CALDAV_URL").unwrap();
        let username = env::var("CALDAV_USERNAME").unwrap();
        let password = env::var("CALDAV_PASSWORD").unwrap();

        let client = CalDAVClient::new(&base_url, &username, &password).await.unwrap();

        for cal_ref in &client.calendars {
            let cal = cal_ref.borrow();
            println!("{}", cal.fancy_name());
            drop(cal);

            let todos = client.get_current_todos(&cal_ref).await.expect("get current todos failed :(");
            for todo in todos.as_ref() {
                let vtodo = todo.vcal.get_vtodo().unwrap();
                let summary = vtodo.get_summary_value().unwrap();
                println!("{}", summary);
            }
            drop(todos);
        }
    }
}
