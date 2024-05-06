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

Random hex string: 813a82e4ac558e001f54b859620358242ff23c32

First part: 813a82e4ac558e001f54b859620358
Second part: 01f54b859620358242ff23c32
Third part: 813a82e4ac558e0242ff23c32

{
First part: 813a82e4ac558e001f54b859620358

Second part: 01f54b859620358242ff23c32

Recovered string: 813a82e4ac558e001f54b859620358242ff23c32
}



{
Second part: 01f54b859620358242ff23c32

Third part: 813a82e4ac558e0242ff23c32

Recovered string: 813a82e4ac558e001f54b859620358242ff23c32
}



{
First part: 813a82e4ac558e001f54b859620358

Third part: 813a82e4ac558e0242ff23c32

Recovered string: 813a82e4ac558e001f54b859620358242ff23c32
}




---

Random hex string: 7268f98d872b93e85acbd6d9397c81f480255427

First part: 7268f98d872b93e85acbd6d9397c81
Second part: 85acbd6d9397c81f480255427
Third part: 7268f98d872b93ef480255427

{
First part: 7268f98d872b93e85acbd6d9397c81

Second part: 85acbd6d9397c81f480255427

Recovered string: 7268f98d872b93e85acbd6d9397c81f480255427
}



{
Second part: 85acbd6d9397c81f480255427

Third part: 7268f98d872b93ef480255427

Recovered string: 7268f98d872b93e85acbd6d9397c81f480255427
}



{
First part: 7268f98d872b93e85acbd6d9397c81

Third part: 7268f98d872b93ef480255427

Recovered string: 7268f98d872b93e85acbd6d9397c81f480255427
}




{
First part: 7268f98d872b93e85acbd6d9397c81

Third part: 7268f98d872b93ef480255427

Recovered string: 7268f98d872b93e85acbd6d9397c81f480255427
}



