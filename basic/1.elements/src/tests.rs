use crate::*;

/** Tests: DO NOT MODIFY */
#[cfg(test)]
mod tests {
    use holochain_types::fixt::*;
    extern crate fixt;

    use super::*;

    #[test]
    fn header_address() {
        let element = ElementFixturator::new(fixt::Predictable).next().unwrap();
        let h = element.header_address().clone();
        let element_hash = get_header_hash(element);
        assert_eq!(h.clone(), element_hash);
    }

    #[test]
    fn test_get_header_timestamp() {
        let element = ElementFixturator::new(fixt::Predictable).next().unwrap();
        let timestamp = element.header().timestamp().clone();

        assert_eq!(timestamp, get_header_timestamp(element));
    }

    #[test]
    fn test_contains_entry() {
        let entry = EntryFixturator::new(fixt::Predictable).next().unwrap();

        let with_entry = ElementFixturator::new(entry).next().unwrap();
        let without_entry = ElementFixturator::new(fixt::Empty).next().unwrap();

        assert_eq!(true, contains_entry(with_entry));
        assert_eq!(false, contains_entry(without_entry));
    }

    #[test]
    fn test_is_header_subconscious() {
        let mut header_fixt = HeaderFixturator::new(fixt::Predictable);

        assert_eq!(true, is_header_subconscious(header_fixt.next().unwrap()));
        assert_eq!(true, is_header_subconscious(header_fixt.next().unwrap()));
        assert_eq!(true, is_header_subconscious(header_fixt.next().unwrap()));
        assert_eq!(false, is_header_subconscious(header_fixt.next().unwrap()));
        assert_eq!(false, is_header_subconscious(header_fixt.next().unwrap()));
        assert_eq!(true, is_header_subconscious(header_fixt.next().unwrap()));
        assert_eq!(true, is_header_subconscious(header_fixt.next().unwrap()));
        assert_eq!(false, is_header_subconscious(header_fixt.next().unwrap()));
        assert_eq!(false, is_header_subconscious(header_fixt.next().unwrap()));
        assert_eq!(false, is_header_subconscious(header_fixt.next().unwrap()));
    }
}
