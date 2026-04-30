#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Symbol, Address};

    #[test]
    fn test_happy_path() {
        let env = Env::default();

        let buyer = Address::random(&env);
        let seller = Address::random(&env);

        EscrowCart::create_escrow(env.clone(), Symbol::short("e1"), buyer.clone(), seller, 100);
        EscrowCart::release(env.clone(), Symbol::short("e1"), buyer);

        let escrow = EscrowCart::get(env, Symbol::short("e1"));
        assert!(escrow.released);
    }

    #[test]
    #[should_panic]
    fn test_unauthorized_release() {
        let env = Env::default();

        let buyer = Address::random(&env);
        let attacker = Address::random(&env);

        EscrowCart::create_escrow(env.clone(), Symbol::short("e1"), buyer.clone(), attacker.clone(), 100);
        EscrowCart::release(env, Symbol::short("e1"), attacker);
    }

    #[test]
    fn test_state_verification() {
        let env = Env::default();

        let buyer = Address::random(&env);
        let seller = Address::random(&env);

        EscrowCart::create_escrow(env.clone(), Symbol::short("e1"), buyer.clone(), seller, 100);

        let escrow = EscrowCart::get(env, Symbol::short("e1"));
        assert_eq!(escrow.amount, 100);
        assert!(!escrow.released);
    }

    #[test]
    fn test_multiple_escrows() {
        let env = Env::default();

        let a = Address::random(&env);
        let b = Address::random(&env);

        EscrowCart::create_escrow(env.clone(), Symbol::short("e1"), a.clone(), b.clone(), 50);
        EscrowCart::create_escrow(env.clone(), Symbol::short("e2"), a, b, 75);

        let e1 = EscrowCart::get(env.clone(), Symbol::short("e1"));
        let e2 = EscrowCart::get(env, Symbol::short("e2"));

        assert_eq!(e1.amount, 50);
        assert_eq!(e2.amount, 75);
    }

    #[test]
    fn test_release_flag() {
        let env = Env::default();

        let buyer = Address::random(&env);
        let seller = Address::random(&env);

        EscrowCart::create_escrow(env.clone(), Symbol::short("e1"), buyer.clone(), seller, 100);
        EscrowCart::release(env.clone(), Symbol::short("e1"), buyer);

        let escrow = EscrowCart::get(env, Symbol::short("e1"));
        assert!(escrow.released);
    }
}