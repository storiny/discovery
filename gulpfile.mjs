import {src, dest, parallel} from "gulp";
import clean_css from "gulp-clean-css";

const minify_css = () => {
    return src('stylesheets/*.css')
        .pipe(clean_css({compatibility: '*'}))
        .pipe(dest('static/vendor/stylesheets'));
};

const build = parallel(minify_css);

export default build;