# synthetic-staking-seward
This repository implements a staking reward system using Synthetic's algorithm with Scrypto programming language. 
ir is created within radixdlt-scrypto environment that you could find [here]([https://pages.github.com/](https://github.com/radixdlt/radixdlt-scrypto)https://github.com/radixdlt/radixdlt-scrypto).

# how is it works?
the algorithm it is similar with synthetic's :
$$r\left(u,a,b\right) = \sum_{t=a}^b R\frac{l\left(u,t\right)}{L\left(t\right)}$$
but since this equation takes many operation which lead to consumes a lot of gas fees we instead will ue this equation :
$$=Rk\left(\sum_{t=0}^b \frac{1}{L(t)} - \sum_{t=0}^{a-1} \frac{1}{L(t)}\right)$$

which :
$$r\left(u,a,b\right)$$
is a function to calculate staking rewards for u (user) a<=u<=b. 
R
is a reward per epoch.
L(t) 
is a total staked token at epoch t.
l(u,t)
is a total user's staked token at epoch t.
