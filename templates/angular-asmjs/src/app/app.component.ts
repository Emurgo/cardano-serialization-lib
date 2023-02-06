import { Component } from '@angular/core';
import {BigNum} from "@emurgo/cardano-serialization-lib-asmjs";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'csl-angular-test-asmjs';
  bigNumValue = BigNum.from_str("333").to_json();
}
