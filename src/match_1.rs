struct BackstageTicket {
    holder_name: String,
    price: u32
}

struct VipTicket {
    holder_name: String,
    price: u32
}

struct StandardTicket {
    price: u32
}

enum Tickets {
    Backstage(BackstageTicket),
    Vip(VipTicket),
    Standard(StandardTicket)
}

impl Tickets {
    fn format(&self) -> String {
        match self {
            Tickets::Backstage(ticket) => format!(
                "[backstage]: {}, price {}",
                ticket.holder_name,
                ticket.price,
            ),
            Tickets::Standard(ticket) => format!(
                "[standard]: price {}",
                ticket.price,
            ),
            Tickets::Vip(ticket) => format!(
                "[vip]: {}, price {}",
                ticket.holder_name,
                ticket.price,
            ),
        }
    }
}

pub fn run_example() {
    let tickets = vec![
        Tickets::Backstage(
            BackstageTicket{
                holder_name: String::from("John Doe"),
                price: 200,
            },
        ),
        Tickets::Vip(
            VipTicket{
                holder_name: String::from("Adam Smith"),
                price: 150,
            },
        ),
        Tickets::Standard(
            StandardTicket{
                price: 80,
            },
        ),
    ];

    for ticket in tickets {
        println!("{}", ticket.format())
    }
}
