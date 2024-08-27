import random
from typing import List, Dict, Optional

# Constants
MAX_ITEMS = 100
COLORS = ["red", "blue", "green", "yellow", "purple"]

class InventoryItem:
    def __init__(self, name: str, quantity: int, price: float):
        self.name = name
        self.quantity = quantity
        self.price = price

    def __repr__(self) -> str:
        return f"InventoryItem(name='{self.name}', quantity={self.quantity}, price={self.price:.2f})"

def generate_inventory(num_items: int) -> List[InventoryItem]:
    """
    Generate a list of random inventory items.
    """
    return [
        InventoryItem(
            name=f"Item_{i}",
            quantity=random.randint(1, 50),
            price=random.uniform(1.0, 100.0)
        )
        for i in range(num_items)
    ]

def calculate_total_value(inventory: List[InventoryItem]) -> float:
    return sum(item.quantity * item.price for item in inventory)

def main():
    inventory = generate_inventory(MAX_ITEMS)
    total_value = calculate_total_value(inventory)
    
    print(f"Total inventory value: ${total_value:.2f}")
    
    # List comprehension with conditional
    expensive_items = [item for item in inventory if item.price > 50.0]
    
    # Dictionary comprehension
    item_quantities = {item.name: item.quantity for item in inventory}
    
    # Lambda function
    sort_by_price = lambda item: item.price
    most_expensive = max(inventory, key=sort_by_price)
    
    print(f"Most expensive item: {most_expensive}")

if __name__ == "__main__":
    main()