# Synthetic Staking Seward
This repository is an implementation of a staking reward system using Synthetic's algorithm with the Scrypto programming language. It is created within the Radixdlt-Scrypto environment, which you can find [here](https://github.com/radixdlt/radixdlt-scrypto).

# how is it works?
The algorithm is similar to Synthetic's:
$$r\left(u,a,b\right) = \sum_{t=a}^b R\frac{l\left(u,t\right)}{L\left(t\right)}$$
However, due to the gas fees associated with the operations in this equation, we have opted for a more efficient alternative:
$$=Rk\left(\sum_{t=0}^b \frac{1}{L(t)} - \sum_{t=0}^{a-1} \frac{1}{L(t)}\right)$$

In this equation,
$$r\left(u,a,b\right)$$
is a function to calculate staking rewards for user (u) where a<=u<=b. 
- R is a reward per epoch.
- L(t) is a total staked token at epoch t.
- l(u,t) is a total user's staked token at epoch t.
