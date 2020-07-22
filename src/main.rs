use bencode::bencode_parse;

fn main() {
    let res = bencode_parse(
        "li12e4:abcdli-23ei34eei4200000024e6:qwertyi-42ed3:\
         foo4:spam3:bari42e6:nestedd3:baz4:boom3:zooi42eeee",
    );

    println!("{:#?}", res);
}
