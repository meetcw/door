import './index.less'

import Header from '../share/header'
import '../share/footer'
import Comment from '../share/comment'

Header.init()
Comment.init()
document.querySelector('.post-wrapper .side .toggle').onclick = function () {
    document.querySelector('.post-wrapper').classList.toggle('with-side')
}