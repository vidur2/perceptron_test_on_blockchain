import Head from 'next/head';
import styles from "../styles/Home.module.css"

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
            let full_split = new Array
            for(let i = 0; i < infor.length; i++){
                split_row = infor[i].split(',');
                full_split.push(split_row)
            }
            console.log(full_split[1][1])
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