# /germinate/templates wll be created at this path
TEMPLATE_PATH=~/clitools

# use debug for development, release for production
BUILD=debug # options: debug, release

if [$BUILD eq release] 
then 
cargo build --release

else
cargo build
fi

echo "Building germinate in $BUILD mode" 

echo "Removing old files from $TEMPLATE_PATH/germinate"
rm -rf ${TEMPLATE_PATH}/germinate/templates 

echo "Copying new files to $TEMPLATE_PATH/germinate"	
cp -p target/debug/germinate ${TEMPLATE_PATH}/germinate 
cp -rf templates ${TEMPLATE_PATH}/germinate
