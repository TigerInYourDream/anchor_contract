alias b:=build
alias d:=deploy
alias t:=test

build:
    @echo "Building..."
    anchor build

deploy:
    @echo "deploy..."
    anchor deploy 

test:
    @echo "test..."
    anchor test 


