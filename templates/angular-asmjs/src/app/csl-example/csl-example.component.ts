import { Component, OnInit } from '@angular/core';
import { BigNum } from "@emurgo/cardano-serialization-lib-asmjs";

@Component({
  selector: 'app-csl-example',
  templateUrl: './csl-example.component.html',
  styleUrls: ['./csl-example.component.css']
})
export class CslExampleComponent implements OnInit {
  enteredValue: string = "";
  hexValue: string = "";
  jsonValue: string = "";

  constructor() { }

  ngOnInit(): void {
  }

  containsOnlyNumbers = (strValue: any) => {
    return /^\d+$/.test(strValue);
  }

  getBigNumValue = (strValue: any) => {
    return BigNum.from_str(strValue)
  };

  toBigNum = (inputEvent: any) => {
    this.enteredValue = inputEvent;
    if (this.containsOnlyNumbers(inputEvent)) {
      this.hexValue = this.getBigNumValue(inputEvent).to_hex();
      this.jsonValue = this.getBigNumValue(inputEvent).to_json();
    } else {
      this.hexValue = "unexpected value";
      this.jsonValue = "unexpected value";
    }
  }

}
