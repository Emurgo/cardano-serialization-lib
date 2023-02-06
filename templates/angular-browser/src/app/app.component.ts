import { Component } from '@angular/core';
import {BigNum} from "@emurgo/cardano-serialization-lib-browser";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'cls-angular-test';
  bigNumValue = BigNum.from_str("333").to_json();
}
