use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Account {
  id: Option<i32>,
  name: String,
  password: String,
  dt: Option<String>,
}

pub fn teste() -> Result<()> {
  let url = "mysql://root:password@localhost:3306/idavoll";

  let opts = Opts::from_url(url)?;
  let pool = Pool::new(opts)?;

  let mut conn = pool.get_conn()?;

  let accounts = vec![
    Account {
      id: None,
      name: "Teste 1".into(),
      password: "sdqwewqe".into(),
      dt: None,
    },
    Account {
      id: None,
      name: "Teste 3".into(),
      password: "azxccqw".into(),
      dt: None,
    },
    Account {
      id: None,
      name: "Teste 5".into(),
      password: "czxcasda".into(),
      dt: None,
    },
    Account {
      id: None,
      name: "Teste 7".into(),
      password: "sdqweqtty".into(),
      dt: None,
    },
    Account {
      id: None,
      name: "Teste 9".into(),
      password: "sdfdsf".into(),
      dt: None,
    },
  ];

  // Now let's insert accounts to the database
  conn.exec_batch(
    r"INSERT INTO teste (name, password)
      VALUES (:name, :password)",
    accounts.iter().map(|p| {
      params! {
        "name" => p.name,
        "password" => p.password
      }
    }),
  )?;

  // Let's select accounts from database. Type inference should do the trick here.
  let selected_payments = conn.query_map(
    "SELECT id, name, password, dt from teste",
    |(id, name, password, dt)| Account {
      id,
      name,
      password,
      dt,
    },
  )?;

  // Let's make sure, that `accounts` equals to `selected_accounts`.
  // Mysql gives no guaranties on order of returned rows
  // without `ORDER BY`, so assume we are lucky.
  assert_eq!(accounts, selected_payments);
  println!("Yay!");

  Ok(());
}
