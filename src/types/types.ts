interface BillItem {
    icon: string,
    title: string,
    subTitle: string,
    cost: number,
    costType: number,
    time: string,
    comment?: string
}

interface Transaction {
    id: number,
    ledger_id: number,
    category_id: number,
    amount: number,
    comment: string,
    transaction_time: string,

    ///
    category_name: string,
    icon: string,
}