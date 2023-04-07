pub struct State<W>
where
    W: std::io::Write,
{
    pub writer: W,
    nesting: usize,
    print_comma: bool,
}

pub trait JsonValue<W>
where
    W: std::io::Write,
{
    fn write(json: &mut State<W>, value: Self) -> Result<(), std::io::Error>;
}

pub trait Json<W>
where
    W: std::io::Write,
{
    fn array(&mut self) -> Result<Array<W>, std::io::Error>;
    fn object(&mut self) -> Result<Object<W>, std::io::Error>;
    fn set<T>(&mut self, value: T) -> Result<(), std::io::Error>
    where
        T: JsonValue<W>;
}

pub fn with_output<W>(writer: W) -> State<W>
where
    W: std::io::Write,
{
    State::<W> {
        writer,
        print_comma: false,
        nesting: 0,
    }
}

impl<W> State<W>
where
    W: std::io::Write,
{
    fn end(&mut self, ending: &str) -> Result<(), std::io::Error> {
        write!(self.writer, "{}", ending)?;
        self.nesting -= 1;
        if self.nesting > 0 {
            self.print_comma = true;
        }
        Ok(())
    }
}

impl<W> Json<W> for State<W>
where
    W: std::io::Write,
{
    fn set<T>(&mut self, value: T) -> Result<(), std::io::Error>
    where
        T: JsonValue<W>,
    {
        if self.print_comma {
            write!(self.writer, ",")?
        }
        T::write(self, value)?;
        if self.nesting > 0 {
            self.print_comma = true
        }
        Ok(())
    }

    fn object(&mut self) -> Result<Object<W>, std::io::Error> {
        if self.print_comma {
            write!(self.writer, ",")?;
            self.print_comma = false;
        }
        write!(self.writer, "{{")?;
        self.nesting += 1;
        Ok(Object { json: self })
    }

    fn array(&mut self) -> Result<Array<W>, std::io::Error> {
        if self.print_comma {
            write!(self.writer, ",")?;
            self.print_comma = false;
        }
        write!(self.writer, "[")?;
        self.nesting += 1;
        Ok(Array { json: self })
    }
}

impl<W> JsonValue<W> for ()
where
    W: std::io::Write,
{
    fn write(json: &mut State<W>, _: Self) -> Result<(), std::io::Error> {
        write!(json.writer, "null")
    }
}

impl<W> JsonValue<W> for bool
where
    W: std::io::Write,
{
    fn write(json: &mut State<W>, boolean: Self) -> Result<(), std::io::Error> {
        write!(json.writer, "{}", boolean)
    }
}

impl<W> JsonValue<W> for f32
where
    W: std::io::Write,
{
    fn write(json: &mut State<W>, number: Self) -> Result<(), std::io::Error> {
        // TOOD Check if always provided number produces right JSON number
        write!(json.writer, "{}", number)
    }
}

impl<W> JsonValue<W> for &str
where
    W: std::io::Write,
{
    fn write(json: &mut State<W>, value: Self) -> Result<(), std::io::Error> {
        let mut value = &value[..];

        write!(json.writer, "\"")?;
        while let Some(split) = value.find('"') {
            write!(json.writer, "{}\\\"", &value[..split])?;
            value = &value[(split + 1)..];
        }
        write!(json.writer, "{}\"", value)?;

        Ok(())
    }
}

pub struct Object<'json, W>
where
    W: std::io::Write,
{
    json: &'json mut State<W>,
}

impl<'object, W> Object<'object, W>
where
    W: std::io::Write,
{
    pub fn key(&mut self, key: &str) -> Result<(), std::io::Error> {
        self.json.set(key)?;
        self.json.print_comma = false;
        write!(self.json.writer, ":")
    }
}

impl<'json, W> Drop for Object<'json, W>
where
    W: std::io::Write,
{
    fn drop(&mut self) {
        self.json.end("}").unwrap()
    }
}

impl<W> Json<W> for Object<'_, W>
where
    W: std::io::Write,
{
    fn array(&mut self) -> Result<Array<W>, std::io::Error> {
        self.json.array()
    }
    fn object(&mut self) -> Result<Object<W>, std::io::Error> {
        self.json.object()
    }
    fn set<T>(&mut self, value: T) -> Result<(), std::io::Error>
    where
        T: JsonValue<W>,
    {
        self.json.set(value)
    }
}

pub struct Array<'json, W>
where
    W: std::io::Write,
{
    json: &'json mut State<W>,
}

impl<'json, W> Drop for Array<'json, W>
where
    W: std::io::Write,
{
    fn drop(&mut self) {
        self.json.end("]").unwrap()
    }
}

impl<W> Json<W> for Array<'_, W>
where
    W: std::io::Write,
{
    fn array(&mut self) -> Result<Array<W>, std::io::Error> {
        self.json.array()
    }
    fn object(&mut self) -> Result<Object<W>, std::io::Error> {
        self.json.object()
    }
    fn set<T>(&mut self, value: T) -> Result<(), std::io::Error>
    where
        T: JsonValue<W>,
    {
        self.json.set(value)
    }
}

#[test]
fn empty_array() {
    let mut out = std::io::BufWriter::new(vec![]);
    {
        let mut json = with_output(&mut out);
        let _object = json.array();
    }
    assert_eq!(out.buffer(), b"[]");
}

#[test]
fn empty_object() {
    let mut out = std::io::BufWriter::new(vec![]);
    {
        let mut json = with_output(&mut out);
        let _object = json.object();
    }
    assert_eq!(out.buffer(), b"{}");
}

#[test]
fn null_and_booleans() {
    let mut out = std::io::BufWriter::new(vec![]);
    {
        let mut json = with_output(&mut out);
        assert!(json.set(()).is_ok());
    }
    assert_eq!(out.buffer(), b"null");

    let mut out = std::io::BufWriter::new(vec![]);
    {
        let mut json = with_output(&mut out);
        assert!(json.set(true).is_ok());
    }
    assert_eq!(out.buffer(), b"true");

    let mut out = std::io::BufWriter::new(vec![]);
    {
        let mut json = with_output(&mut out);
        assert!(json.set(false).is_ok());
    }
    assert_eq!(out.buffer(), b"false");
}

#[test]
fn numbers() {
    let mut out = std::io::BufWriter::new(vec![]);
    {
        let mut json = with_output(&mut out);
        assert!(json.set(123.456).is_ok());
    }
    assert_eq!(out.buffer(), b"123.456");
}

#[test]
fn strings() {
    let mut out = std::io::BufWriter::new(vec![]);
    {
        let mut json = with_output(&mut out);
        assert!(json.set("hello, world").is_ok());
    }
    assert_eq!(out.buffer(), b"\"hello, world\"");

    let mut out = std::io::BufWriter::new(vec![]);
    {
        let mut json = with_output(&mut out);
        assert!(json
            .set("One great person once wrote: \"hello, world\"")
            .is_ok());
    }
    assert_eq!(
        std::str::from_utf8(out.buffer()).unwrap(),
        "\"One great person once wrote: \\\"hello, world\\\"\""
    );
}

#[test]
fn array_with_elements() {
    let mut out = std::io::BufWriter::new(vec![]);
    {
        let mut json = with_output(&mut out);
        let mut array = json.array().unwrap();
        assert!(array.set(1.0).is_ok());
        assert!(array.set(2.0).is_ok());
        assert!(array.set(3.0).is_ok());
    }
    assert_eq!(out.buffer(), b"[1,2,3]");
}

#[test]
fn nested_array() {
    let mut out = std::io::BufWriter::new(vec![]);
    {
        let mut json = with_output(&mut out);
        let mut array = json.array().unwrap();
        {
            let mut array = array.array().unwrap();
            assert!(array.set(1.0).is_ok());
            assert!(array.set(2.0).is_ok());
        }
        {
            let mut array = array.array().unwrap();
            assert!(array.set(3.0).is_ok());
            assert!(array.set(4.0).is_ok());
            assert!(array.set(5.0).is_ok());
        }
    }
    assert_eq!(
        std::str::from_utf8(out.buffer()).unwrap(),
        "[[1,2],[3,4,5]]"
    );
}

#[test]
fn object_with_elements() {
    let mut out = std::io::BufWriter::new(vec![]);
    {
        let mut json = with_output(&mut out);
        let mut array = json.object().unwrap();
        assert!(array.key("foo").is_ok());
        assert!(array.set(1.0).is_ok());

        assert!(array.key("bar").is_ok());
        assert!(array.set(2.0).is_ok());
    }
    assert_eq!(
        std::str::from_utf8(out.buffer()).unwrap(),
        "{\"foo\":1,\"bar\":2}"
    );
}
