use crate::error::Error;
use crate::store::PasswordStore;

pub fn show(store: PasswordStore, item: &str, copy_to_clipboard: bool) -> Result<(), Error> {
    let secret = store.get(item)?;

    println!("{}", secret);
    Ok(())
}
