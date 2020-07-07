# Generating CDDL instances

First you need to install `cddl`
```
sudo apt install ruby
sudo gem install cddl
sudo gem install cbor-diag
```

You can generate new tests with
1) `cddl specs/shelley.cddl generate 1 > test/name_here.diag`
2) `diag2cbor.rb test/name_here.diag > test/name_here.cbor`

You can combine these together with `cddl specs/shelley.cddl generate 1 | diag2cbor.rb > test/name_here.cbor`
