import Head from 'next/head';
import styles from "../styles/Home.module.css"

// Use of nearAPI
async function contractUse(inputs, outputs){
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
    await perceptron.train({"inputs": inputs, "outputs": outputs});
  
  }

export default function train(){
    const form = async event => {
        event.preventDefault();
        const file = document.getElementById("fileInfo").files[0];
        console.log(file)
        const file_reader = new FileReader
        file_reader.readAsText(file)
        file_reader.onload = async function(e){
            let infor = e.target.result.toString()
            infor = infor.split("\n")
            let split_row = new Array;
            let inputArray = new Array;
            let outputArray = new Array;
            let counter = 0;
            for(let i = 1; i < infor.length; i++){
                split_row = infor[i].split(',');
                inputArray.push([parseInt(split_row[0]), parseInt(split_row[1]), parseInt(split_row[2]), parseInt(split_row[3])]);
                outputArray.push(parseInt(split_row[4]))
                counter++;
                if(counter == 10){
                    contractUse(inputArray, outputArray)
                    inputArray = new Array
                    counter = 0;
                }
            }
        }
    }
    return(
        <div className={styles.container}>
            <h1>Sheep Deep Learn: Train</h1>
            <p>Upload a csv here:</p>
            <form onSubmit={form}>
                <input type="file"  accept="csv" id="fileInfo"/>
                <button type="submit">Submit file</button>
            </form>
        </div>
    )
}