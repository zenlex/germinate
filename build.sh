# /germinate/templates wll be created at this path
GERMINATE_PATH=~/clitools/germinate

# use debug for development, release for production
BUILD=debug # options: debug, release

if [ $BUILD = release ] 
then 
cargo build --release

else
cargo build
fi

echo "Building germinate in $BUILD mode" 

echo "Removing old files from $GERMINATE_PATH"
rm -rf ${GERMINATE_PATH}/templates 

echo "Copying new files to $GERMINATE_PATH"	
cp -p target/debug/germinate ${GERMINATE_PATH}
cp -rf templates ${GERMINATE_PATH}
