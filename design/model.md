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
- doc number

## Estimate
todo!

## Credit 
todo!

## SalesReceipt
todo!

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