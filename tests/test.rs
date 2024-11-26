
#[cfg(test)]
mod tests {

    #[test]
    fn test_debug() {
        let dex = dex::DexReader::from_file("./resources/custom.dex");
        dbg!(dex.map(|s| s.inner).unwrap());
    }

}
