use eip_712_derive::*;
// No salt
struct DomainStruct {
    name: String,
    version: String,
    chain_id: U256,
    verifying_contract: Address,
}

impl StructType for DomainStruct {
    const TYPE_NAME: &'static str = "EIP712Domain";
    fn visit_members<T: MemberVisitor>(&self, visitor: &mut T) {
        visitor.visit("name", &self.name);
        visitor.visit("version", &self.version);
        visitor.visit("chainId", &self.chain_id);
        visitor.visit("verifyingContract", &self.verifying_contract);
    }
}

struct Person {
    name: String,
    wallet: Address,
}
impl StructType for Person {
    const TYPE_NAME: &'static str = "Person";
    fn visit_members<T: MemberVisitor>(&self, visitor: &mut T) {
        visitor.visit("name", &self.name);
        visitor.visit("wallet", &self.wallet);
    }
}

struct Mail {
    from: Person,
    to: Person,
    contents: String,
}
impl StructType for Mail {
    const TYPE_NAME: &'static str = "Mail";
    fn visit_members<T: MemberVisitor>(&self, visitor: &mut T) {
        visitor.visit("from", &self.from);
        visitor.visit("to", &self.to);
        visitor.visit("contents", &self.contents);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut chain_id = U256([0_u8; 32]);
    chain_id.0[31] = 1;

    let domain = DomainStruct {
        name: "Ether Mail".to_owned(),
        version: "1".to_owned(),
        chain_id,
        verifying_contract: Address(
            (&(hex::decode("CcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC").unwrap())[..])
                .try_into()
                .unwrap(),
        ),
    };
    let domain_separator = DomainSeparator::new(&domain);

    let message = Mail {
        from: Person {
            name: "Cow".to_owned(),
            wallet: Address(
                (&(hex::decode("CD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826").unwrap())[..])
                    .try_into()
                    .unwrap(),
            ),
        },
        to: Person {
            name: "Bob".to_owned(),
            wallet: Address(
                (&(hex::decode("bBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB").unwrap())[..])
                    .try_into()
                    .unwrap(),
            ),
        },
        contents: "Hello, Bob!".to_owned(),
    };

    let pk = keccak_hash::keccak("cow").to_fixed_bytes();

    let result = sign_typed(&domain_separator, &message, &pk).unwrap();
    let mut serialized = Vec::new();
    serialized.extend_from_slice(&result.0);
    serialized.push(result.1);
    let result = hex::encode(&serialized);
    println!("Signature:{}", result);
    let expected = "4355c47d63924e8a72e509b65029052eb6c299d53a04e167c5775fd466751c9d07299936d304c153f6443dfa05f40ff007d72911b6f72307f996231605b915621c";

    assert_eq!(expected, result);

    Ok(())
}