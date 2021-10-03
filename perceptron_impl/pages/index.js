import Head from 'next/head';
import styles from "../styles/Home.module.css"

// Use of nearAPI
async function contractUse(height, weight, legs, state){
  const { Contract, WalletConnection, keyStores, connect } = require("near-api-js");
  const methodOptions = {
    viewMethods: ['predict'],
    changeMethods: ['train']
  }

const config = {
  networkId: "testnet",
  keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://wallet.testnet.near.org",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://explorer.testnet.near.org",
};
  const near = await connect(config);
  const wallet = new WalletConnection(near);
  const perceptron = new Contract(
    wallet.account(),
    'test2.perceptron.testnet',
    methodOptions
  )
  const prediction = await perceptron.predict({"height": height, "weight": weight, "legs": legs, "is_alive": state});
  console.log("Thing is")
  console.log(await prediction)
  return await prediction
}


export default function Home() {
  const params = async event => {
    // Gets document elements by id
    let height = document.getElementById('height').value
    let weight = document.getElementById('weight').value
    let legs = document.getElementById('legs').value    
    const state = document.getElementById('state').value

    // Casts vars to int
    height = parseInt(height)
    weight = parseInt(weight)
    legs = parseInt(legs)

    let sheepState;
    // Changes state to bool
    if(state.toLowerCase() == 'alive'){
      sheepState = 1;
    }else{
      sheepState = 0;
    }
    if (weight != null || height == null || legs == null || sheepState == null){
      const output = document.getElementById('prediction');
      output.innerHTML = "Please fill in all fields!"
    }else{
      contractUse(height, weight, legs, sheepState).then(value => {
        const output = document.getElementById('prediction');
        if (value == 1){
          output.innerHTML = "Perceptron has determined it was a sheep!"
        }else{
          output.innerHTML = "Perceptron has determined it wasn't a sheep.<p>Think the prediction was innacurate?</p><p>Train it with your own data <a href='http://localhost:3000/train'>here</a>!</p>"
        }
      })
    }
    
  }
  return (
    <div className={styles.container}>
      <Head>
        <title>Sheep Deep Learning</title>
      </Head>
      <h1>Deep Sheep Learning</h1>
      <p>Enter thing's height: </p>
      <input type= "text" required id='height'></input>
      <p>Enter thing's weight: </p>
      <input type= "text" required id='weight'></input>
      <p>Enter thing's amount of legs: </p>
      <input type= "text" required id='legs'></input>
      <p>Enter whether thing is alive: </p>
      <input type= "text" required id='state'></input>
      <button onClick={params}>Click to predict </button>
      <p id="prediction"></p>
    </div>
  )
}
