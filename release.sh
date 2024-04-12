if [ $1 = "prod" ];
then RELEASE_TYPE="prod"
elif [ $1 = "beta" ];
then RELEASE_TYPE="beta"
else
  echo "First parameter is expected 'prod' or 'beta'"
  return 1
fi

echo "Preparing ${RELEASE_TYPE} release"

. build-and-test.sh \
&& npm run js:publish-nodejs:${RELEASE_TYPE} \
&& npm run js:publish-browser:${RELEASE_TYPE} \
&& npm run js:publish-asm:${RELEASE_TYPE} \
&& (cd rust; cargo publish --allow-dirty)
