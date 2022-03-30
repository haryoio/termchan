#[cfg(test)]
mod tests {
    use termch::headers;

    #[test]
    fn test_cookie_single() {
        let cookie_vec = vec![("yuki", "akari")];
        let cookie = headers::cookie_from_vec(cookie_vec);
        assert_eq!(cookie, "yuki=akari");
    }

    #[test]
    fn test_cookie_multiple() {
        let cookie_vec = vec![("yuki", "akari"), ("yuki2", "akari2")];
        let cookie = headers::cookie_from_vec(cookie_vec);
        assert_eq!(cookie, "yuki=akari; yuki2=akari2");
    }
}
