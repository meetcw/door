import './index.less'

import Header from '../share/header'
import '../share/footer'

Header.init()
document.querySelector('#toggle-silde-button').onclick = function () {
    document.body.classList.toggle('show-slide')
}
