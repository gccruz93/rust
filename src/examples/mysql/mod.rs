use mysql::prelude::*;
use mysql::*;

pub fn main() {
  let url = "mysql://root:123456@localhost:3306/idavoll";
  let opts = Opts::from_url(url).unwrap();
  let pool = Pool::new(opts).unwrap();
  let mut conn = pool.get_conn().unwrap();
  conn
    .exec_drop(
      r"INSERT INTO teste (name, password)
      VALUES (:name, :password)",
      params! {
          "name" => "Teste",
          "password" => "asddssad"
      },
    )
    .unwrap();

  // conn
  //   .query_iter("SELECT id, name, password, dt from teste")
  //   .unwrap()
  //   .for_each(|row| {
  //     let r: (i32, String, String, NaiveDate) = from_row(row.unwrap());
  //     println!(
  //       "id: {}, name: {}, password: {}, dt: {:?}",
  //       r.0, r.1, r.2, r.3
  //     );
  //   });
}
