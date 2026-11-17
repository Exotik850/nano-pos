# nano-pos Models

nano-pos is meant to be a minimal implementation of a Point-of-Sale system for small businesses, so only the following is modeled directly:

- Users 

- Contact
  - Address

- Invoice
- Estimate
- Credit
- SalesReceipt
- Payment
- Product


## User
- username
- email
- password

## Contact
- first name
- last name
- phone number 
- email 
- address

## Address
- address line(s)
- city
- state
- zip

## Invoice
- customer id
- user id
- doc number
- description?
- order date
- deliver date

### InvoiceProduct
- invoice id
- product id
- quantity
- tax rate?
- price?
- description?

## Estimate
- customer id?
- user id
- doc number 
- description?

## EstimateProduct
- estimate id
- product id
- quantity
- tax rate?
- price?
- description?

## Credit 
- customer id
- user id
- document number
- description?
- credit date

### CreditProduct
- credit id
- product id
- quantity
- tax rate?
- price?
- description?

## SalesReceipt
- customer id
- user id
- document number
- description?
- sales receipt date

### SalesReceiptProduct
- sales receipt id
- product id
- quantity
- tax rate?
- price?
- description?

## Payment
todo!

## Product
- name
- sku
- description
- price
- image
- quantity

There are also extensible fields that are managed with the `CustomField` type, which can be attached to any of the above in order to add columns to those types during runtime

## CustomField
- name
- type
- extends