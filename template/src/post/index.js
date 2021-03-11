import './index.less'

import Header from '../share/header'
import '../share/footer'

Header.init()
document.querySelector('.slide-wrapper .toggle').onclick = function () {
    document.body.classList.toggle('show-slide')
}
