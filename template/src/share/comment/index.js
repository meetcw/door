import 'gitalk/dist/gitalk.css'
import Gitalk from 'gitalk'

export default {
    init: function () {
        var id = document.querySelector('.comment-wrapper #post-id').value
        var gitalk = new Gitalk({
            clientID: 'd6bede35b03c7fe342cd',
            clientSecret: '27886c787d06b73b751f4a71cb08ca132938742a',
            repo: 'baiyan',
            owner: 'obaiyan',
            admin: ['obaiyan'],
            id: id,
            distractionFreeMode: false,
            language: 'zh-CN'
        })
        var element = document.querySelector('.comment-wrapper #comment-container')
        element.type = "string"
        gitalk.render(element)
    }
}