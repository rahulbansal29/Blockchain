mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("Alice pays Bob 10 coins".to_string());
    blockchain.add_block("Bob pays Charlie 5 coins".to_string());

        // 🚨 STEP 7: Tampering with the blockchain
    blockchain.chain[1].data = "HACKED: Alice pays Bob 1000 coins".to_string();


    println!("{:#?}", blockchain.chain);
    println!("Is blockchain valid? {}", blockchain.is_valid());
}
