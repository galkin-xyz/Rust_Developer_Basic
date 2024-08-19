struct Account {
    name: String,
    balance: i64,
}

impl Drop for Account {
    fn drop(&mut self) {
        if self.balance != 0 {
            println!("[{:<5}]: Ненулевой баланс! ({})", &self.name, &self.balance);
        } else {
            println!("[{:<5}]: Аккаунт удалён.", &self.name)
        }
    }
}

fn print_balance(acc: &Account) {
    println!("[{:<5}]: Баланс = {}", &acc.name, &acc.balance);
}

fn transfer_funds(from: &mut Account, to: &mut Account, count: i64) {
    from.balance -= count;
    to.balance += count;
}

fn destroy_account(mut acc: Account, to: &mut Account) {
    if acc.balance != 0 {
        to.balance += acc.balance;
        acc.balance = 0;
    }
}

struct Bank {
    accounts: Vec<Account>,
    credit_rate: u32,
    debit_rate: u32,
}

fn accrue_interest(bank: &mut Bank) {
    for acc in &mut bank.accounts {
        let rate: i64 = match acc.balance {
            1.. => bank.debit_rate as i64,
            ..=-1 => bank.credit_rate as i64,
            0 => 0,
        };
        acc.balance += acc.balance * rate / 100 / 100;
    }
}

struct BankBalance {
    liabilities: u64,
    assets: u64,
}

fn bank_balance(bank: &Bank) -> BankBalance {
    let mut bank_balance = BankBalance {
        liabilities: 0,
        assets: 0,
    };
    for acc in &bank.accounts {
        match acc.balance {
            1.. => bank_balance.liabilities += acc.balance as u64,
            ..=-1 => bank_balance.assets += -acc.balance as u64,
            0 => (),
        }
    }
    bank_balance
}

fn merge_banks(from: Bank, to: &mut Bank) {
    for acc in from.accounts {
        to.accounts.push(acc);
    }
}

fn main() {
    let mut alice = Account {
        name: "alice".to_string(),
        balance: 100,
    };
    let mut bob = Account {
        name: "bob".to_string(),
        balance: -20,
    };
    let mut cris = Account {
        name: "cris".to_string(),
        balance: 0,
    };
    let mut den = Account {
        name: "den".to_string(),
        ..cris
    };

    print_balance(&alice);
    print_balance(&bob);
    print_balance(&cris);
    print_balance(&den);

    transfer_funds(&mut alice, &mut den, 30);
    transfer_funds(&mut bob, &mut den, 50);

    print_balance(&alice);
    print_balance(&bob);
    print_balance(&cris);
    print_balance(&den);

    destroy_account(den, &mut cris);

    let mut bank1: Bank = Bank {
        accounts: Vec::new(),
        credit_rate: 1200,
        debit_rate: 500,
    };
    bank1.accounts.push(alice);
    bank1.accounts.push(bob);

    let mut bank2: Bank = Bank {
        accounts: Vec::new(),
        credit_rate: 1800,
        debit_rate: 1000,
    };
    bank2.accounts.push(cris);

    let mut bb: BankBalance = bank_balance(&bank1);
    println!(
        "[банк1]: Обязательства: {}, активы: {}",
        bb.liabilities, bb.assets
    );
    accrue_interest(&mut bank1);
    bb = bank_balance(&bank1);
    println!(
        "[банк1]: Обязательства: {}, активы: {}",
        bb.liabilities, bb.assets
    );

    bb = bank_balance(&bank2);
    println!(
        "[банк2]: Обязательства: {}, активы: {}",
        bb.liabilities, bb.assets
    );

    merge_banks(bank2, &mut bank1);

    bb = bank_balance(&bank1);
    println!(
        "[банк1+2]: Обязательства: {}, активы: {}",
        bb.liabilities, bb.assets
    );
}

#[cfg(test)]
mod tests {
    use crate::transfer_funds;

    use super::*;

    #[test]
    fn transfer_funds_works() {
        let mut acc_from = Account {
            name: "from".to_string(),
            balance: 100,
        };
        let mut acc_to = Account {
            name: "to".to_string(),
            balance: 20,
        };

        transfer_funds(&mut acc_from, &mut acc_to, 30);

        assert_eq!(acc_from.balance, 70);
        assert_eq!(acc_to.balance, 50);
    }

    #[test]
    fn accrue_interests_works1() {
        let mut bank = Bank {
            accounts: Vec::new(),
            credit_rate: 2000,
            debit_rate: 1000,
        };
        bank.accounts.push(Account {
            name: "deb1".to_string(),
            balance: 100,
        });
        bank.accounts.push(Account {
            name: "deb2".to_string(),
            balance: 200,
        });
        bank.accounts.push(Account {
            name: "cred1".to_string(),
            balance: -100,
        });
        bank.accounts.push(Account {
            name: "cred2".to_string(),
            balance: -200,
        });

        accrue_interest(&mut bank);

        assert_eq!(bank.accounts[0].balance, 110);
        assert_eq!(bank.accounts[1].balance, 220);
        assert_eq!(bank.accounts[2].balance, -120);
        assert_eq!(bank.accounts[3].balance, -240);
    }

    #[test]
    fn accrue_interests_works2() {
        let mut bank = Bank {
            accounts: Vec::new(),
            credit_rate: 80,
            debit_rate: 50,
        };
        bank.accounts.push(Account {
            name: "deb1".to_string(),
            balance: 1000,
        });
        bank.accounts.push(Account {
            name: "deb2".to_string(),
            balance: 2000,
        });
        bank.accounts.push(Account {
            name: "cred1".to_string(),
            balance: -1000,
        });
        bank.accounts.push(Account {
            name: "cred2".to_string(),
            balance: -2000,
        });

        accrue_interest(&mut bank);

        assert_eq!(bank.accounts[0].balance, 1005);
        assert_eq!(bank.accounts[1].balance, 2010);
        assert_eq!(bank.accounts[2].balance, -1008);
        assert_eq!(bank.accounts[3].balance, -2016);
    }

    #[test]
    fn bank_balance_works() {
        let mut bank = Bank {
            accounts: Vec::new(),
            credit_rate: 2000,
            debit_rate: 1000,
        };
        bank.accounts.push(Account {
            name: "deb1".to_string(),
            balance: 1000,
        });
        bank.accounts.push(Account {
            name: "deb2".to_string(),
            balance: 200,
        });
        bank.accounts.push(Account {
            name: "cred1".to_string(),
            balance: -100,
        });
        bank.accounts.push(Account {
            name: "cred2".to_string(),
            balance: -2000,
        });

        let balance: BankBalance = bank_balance(&bank);

        assert_eq!(balance.liabilities, 1200);
        assert_eq!(balance.assets, 2100);
    }

    #[test]
    fn merge_banks_works() {
        let mut from_bank = Bank {
            accounts: Vec::new(),
            credit_rate: 2000,
            debit_rate: 1000,
        };
        from_bank.accounts.push(Account {
            name: "deb1".to_string(),
            balance: 1000,
        });
        from_bank.accounts.push(Account {
            name: "cred2".to_string(),
            balance: -2000,
        });

        let mut to_bank = Bank {
            accounts: Vec::new(),
            credit_rate: 2200,
            debit_rate: 1100,
        };
        to_bank.accounts.push(Account {
            name: "deb2".to_string(),
            balance: 200,
        });
        to_bank.accounts.push(Account {
            name: "cred1".to_string(),
            balance: -100,
        });

        merge_banks(from_bank, &mut to_bank);

        assert_eq!(to_bank.accounts[0].balance, 200);
        assert_eq!(to_bank.accounts[1].balance, -100);
        assert_eq!(to_bank.accounts[2].balance, 1000);
        assert_eq!(to_bank.accounts[3].balance, -2000);

        assert_eq!(to_bank.accounts[0].name, "deb2");
        assert_eq!(to_bank.accounts[1].name, "cred1");
        assert_eq!(to_bank.accounts[2].name, "deb1");
        assert_eq!(to_bank.accounts[3].name, "cred2");
    }
}
