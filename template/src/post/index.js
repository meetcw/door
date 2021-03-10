import './index.less'

import Header from '../share/header'
import '../share/footer'

Header.init()
document.querySelector('.post-wrapper .side .toggle').onclick = function () {
    document.querySelector('.post-wrapper').classList.toggle('with-side')
}
