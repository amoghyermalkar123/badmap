<proto><hash>name\n<key>1\n<d>u64\n<v>user1\n<d>str\n<key>2\n<d>u64\n<v>user2\n<d>str\n</hash></proto>


<p>
    <h>name\n
        <k>+1\n
            <v>:user1\n
        <k>:amogh\n
            <v>:user2\n
    <H>
<P>



enum Token {
    Int
    String
}

enum DataType {
    0 u64
    1 f64
    2 &str
    3 String
}

grammar - Vec<(K,V)>

output - [("hashname", ""),(key1, value1),(key2, value2),(key3, value3) ..]
