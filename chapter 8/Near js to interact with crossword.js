// NEAR JavaScript SDK to interact with the deployed contract.
const near = require("near-api-js");
async function interactWithCrosswordGameContract() {
  const keyStore = new near.keyStores.InMemoryKeyStore();
  const nearConfig = {
    keyStore,
    nodeUrl: "https://rpc.testnet.near.org",
    networkId: "testnet",
    contractName: "your_account_id",
  };
  const near = await near.connect(nearConfig);
  const account = await near.account(nearConfig.contractName);
  const crosswordGameContract = new near.Contract(
    account,
    nearConfig.contractName,
    {
      viewMethods: ["get_game"],
      changeMethods: ["create_game", "submit_word"],
      sender: nearConfig.contractName,
    }
  );
  // Interact with the contract methods
  const gameDetails = await crosswordGameContract.get_game({ game_id: 1 });
  console.log("Game Details:", gameDetails);
  // Add more contract interactions here
}
interactWithCrosswordGameContract();
