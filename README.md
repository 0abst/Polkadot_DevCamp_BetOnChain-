# BetOnChain - Betting on Sport/Esport results

**Introduction**

With the World Cup taking place, it is interesting to have a look at the sports/esports betting market. It is a very large one: More than 75 Billion dollars according to a _grandviewresearch_ study (https://www.grandviewresearch.com/industry-analysis/sports-betting-market-report). Such a market is sometimes considered to be manipulated, not very transparent, etc. The blockchain, with its unique technology, may allow to add transparency into this market and hence, make it more efficient.

**Our idea**

A global architecture can be found below:



![image](https://user-images.githubusercontent.com/92883939/205520437-894f32cd-9473-4877-9465-749796d7a00f.png)



Of course, inputs from external sources are needed in order to have our betting platform working. This is done through Oracles: they provide the ability to bridge the gap between blockchain networks and the outside world. They allow users to send external data to the blockchain, and, as a result, are essential parts of most blockchain applications. Dapps in the DEFI, Risk, betting sectors are not relevant without Oracles.

**Challenges we have encountered**

The main challenge we have had was about the obsolescence of the documentations, i.e. the fact that documentation was not longer up to date at the time of the project. This is valid for:

  * The data providers. We focused on Chainlink, given that it is the largest data provider on Ethereum. The releases Chainlink has done during the last years make the tutorials about Chainlink Polkadot no longer so relevant - only the general structure can be found

  * Dependencies: When we want to test code from non-official tutorials to be more familiar with Substrate, libraries are associated to such code. The libraries are sometimes no longer maintained and, as a result, the code cannot be tested easily

**What we have done**

 * We have focused on defining the base logic: We provide the structure on Substrate to bet on certain matches as well as determine what happens when the match is finished
 
 	* In order to do this, the pallets BetonEvents and Beton have been created
	
 	* A treasury pallet has also been added - in order to act as a kind of pool which would automatically retrocede the payoffs for the betting platform
 
 * We are working on the connection between Chainlink and Substrate. We have Chainlink nodes running on postgresql databases, we have set up the internal initiators, built the Aggregated Price Feed External Adapter, as well as bridged the substrate blockchain with the Chainlink node, but we do not properly get the price feed from the Substrate blockchain
 
**Next steps**
 
  * Making sure we can connect to real chainlink data feed
  
  * Optimise the pallets and win-rates so that this can be actually profitable/sustainable
  
  * Making a proper frontend
  
  * Refreshing the tutorials - to provide a convenient, updated, possibility for new users to use feeds from Chainlink with Substrate

**Main sources**

 * https://github.com/LaurentTrk/chainlink-substrate-tutorial

 * https://docs.substrate.io/tutorials/

 * https://github.com/paritytech/substrate/blob/master/frame/treasury/src/lib.rs

 * https://github.com/paritytech/substrate/blob/master/frame/balances/src/lib.rs

**Team**
* Rostik#1223
* joural#6461
