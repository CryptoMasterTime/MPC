# Multi-Party Computation (MPC) Example

This example demonstrates the concept of Multi-Party Computation (MPC) applied to splitting and recovering a private key. 

## Introduction
MPC is a cryptographic technique that enables multiple parties to jointly compute a function over their inputs while keeping those inputs private. In the context of this example, we split a private key into multiple fragments and distribute them among participants. Through collaboration, any subset of participants can reconstruct the original private key without revealing their individual fragments.

## Example Scenario
Suppose we have a private key "abc". We split it into three fragments: "ab", "bc", and "ac". Any combination of two fragments can be used to reconstruct the original private key "abc".

## Usage
1. Clone the repository.
2. Run the code provided in the example. It will generate three key fragments: "ab", "bc", and "ac".
3. You can experiment with different combinations of fragments to see how they can be used to reconstruct the original private key.

## Example Code
The provided Rust code demonstrates the splitting and recovery of the private key "abc" using MPC techniques. It splits the key into fragments and provides a function to recover the original key from any combination of fragments.

## Credits
This example is provided by CryptoMasterTime.



Test Data:

Random hex string: 6eddc14354016438095273727ba6d3d32f78aab7
First part: 6eddc14354016438095273727ba6d3
Second part: 8095273727ba6d3d32f78aab7
Third part: 6eddc1435401643d32f78aab7

First part: 6eddc14354016438095273727ba6d3
Second part: 8095273727ba6d3d32f78aab7
Recovered string: 6eddc14354016438095273727ba6d3d32f78aab7

Second part: 8095273727ba6d3d32f78aab7
Third part: 6eddc1435401643d32f78aab7
Recovered string: 6eddc14354016436eddc1435401643d32f78aab7

First part: 6eddc14354016438095273727ba6d3
Third part: 6eddc1435401643d32f78aab7
Recovered string: 6eddc14354016438095273727ba6d3d32f78aab7

=============================================================

Finished dev [unoptimized + debuginfo] target(s) in 5.55s
     Running `target/debug/mpc-fix-private-key`
Random hex string: f5af901a93521fc40264304ddf789b13a2f84120
First part: f5af901a93521fc40264304ddf789b
Second part: 40264304ddf789b13a2f84120
Third part: f5af901a93521fc13a2f84120

First part: f5af901a93521fc40264304ddf789b
Second part: 40264304ddf789b13a2f84120
Recovered string: f5af901a93521fc40264304ddf789b13a2f84120

Second part: 40264304ddf789b13a2f84120
Third part: f5af901a93521fc13a2f84120
Recovered string: f5af901a93521fcf5af901a93521fc13a2f84120

First part: f5af901a93521fc40264304ddf789b
Third part: f5af901a93521fc13a2f84120
Recovered string: f5af901a93521fc40264304ddf789b13a2f84120
