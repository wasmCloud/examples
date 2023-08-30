# KVCounter UI

This UI is meant to work in tandem with the KVCounter Actor. It will allow you to set a "bucket" name and increment the
count for this bucket by clicking the button.

## Environment Variables

The `REACT_APP_API_URL` should be the url to the KVCounter API. If it isn't set, it will fall back to the current host.
In development, it assumes that the url will [proxy](https://create-react-app.dev/docs/proxying-api-requests-in-development/)
all missed requests to http://localhost:8080.

Note: The API must support CORS if it is not on the same domain.

## Available Scripts

In the project directory, you can run:

### `npm start`

Runs the app in the development mode.\
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

The page will reload if you make edits.\
You will also see any lint errors in the console.

### `npm run build`

Builds the app for production to the `build` folder.\
It correctly bundles React in production mode and optimizes the build for the best performance.

The build is minified and the filenames include the hashes.\
Your app is ready to be deployed!
