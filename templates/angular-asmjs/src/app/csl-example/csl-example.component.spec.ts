import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CslExampleComponent } from './csl-example.component';

describe('CslExampleComponent', () => {
  let component: CslExampleComponent;
  let fixture: ComponentFixture<CslExampleComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ CslExampleComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CslExampleComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
