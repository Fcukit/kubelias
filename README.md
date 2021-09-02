# kubelias

Данная CLI-утилита предназначена для автоматизации доступа к Kubernetes-_pod_'ам серверов, на которых развернуты различные среды окружения для проекта.

## Проблема
Зачастую на проекте бывает как минимум два удаленных сервера, отвечающих за _production_ и _staging_ среды. А если таких проектов несколько, то количество серверов кратно увеличивается. В случае использования Kubernetes-кластера на каждом сервере могут быть подняты несколько _pod_'ов, причем каждый из них может отвечать только за определенную задачу. Например, для Rails-приложения 2 _pod_'а могут отвечать за сам application-сервер, а 1 _pod_ - за Sidekiq.

Для быстрого доступа к определенному Kubernetes-_pod_'у можно выполнить ряд команд, введенных вручную, после чего можно даже настроить _alias_'ы. Однако, после каждого деплоя на определенном сервере разворачиваются новые Kubernetes-_pod_'ы, из-за чего процесс быстрого доступа к удаленному серверу затрудняется ввиду рутинности процесса.

Пример того, как можно получить список _pod_'ов привычным образом.
`➜   kubectl --kubeconfig=/path-to-config/some-config-file --namespace project-backend get pod`
```
NAME                                         READY   STATUS    RESTARTS   AGE
project-x-backend-813b55d9b3-rse2x           1/1     Running   0          3h17m
project-x-backend-866b45c0b6-pvf6z           1/1     Running   0          3h17m
project-x-backend-sidekiq-8548b78d65-224s9   1/1     Running   0          3h17m
```
После чего можно выделить нужный _pod_ и создать _alias_:
```
alias project-x-stage-bash="kubectl --kubeconfig=/path-to-config/some-config-file -n project-backend exec -it project-backend-866b45c0b6-pvf6z -- bundle exec bash"
alias project-x-stage-console="kubectl --kubeconfig=/path-to-config/some-config-file -n project-backend exec -it project-backend-866b45c0b6-pvf6z -- bundle exec rails console"
```

И процесс обновления _alias_'ов нужно проделывать после каждого деплоя, для каждого окружения, для каждого проекта...
## Решение

Процесс создания _alias_'ов можно автоматизировать с помощью данной утилиты. Благодаря этому всего одной командой можно актуализировать _alias_ для доступа к определенному _pod_'у.

```
➜  kubelias get-pod --config /path-to-config/some-config-file --namespace project-backend

1 :project-x-backend-sidekiq-8548b78d65-224s9
2 :project-x-backend-866b45c9b6-pvf6z
Press number of needed pod or 'q' to quit.
```

```
Your choice: 2
Current pod is project-x-backend-866b45c9b6-pvf6z
```

`➜  kubelias alias project-x-stage-bash bash`

`➜  kubelias alias project-x-stage-console rails console`

```
➜  kubelias project-x-stage-console
irb(main):001:0>
```

Таким образом, можно хранить список _alias_'ов для всех своих проектов, где используется Kubernetes. В случае, если после деплоя развернулись новые _pod_'ы и _alias_'ы для какого-то сервера перестали работать, так как сменился **current_pod**, то запуском всего одной команды будут обновлены вся связанные с этим _"битым"_ _pod_'ом _alias_'ы

`➜  kubelias refresh project-x-stage-bash`

В данном случае будет заново выведен список актуальных _pod_'ов и после выбора нужного - автоматически обновятся два _alias_'а:
- `project-x-stage-bash`
- `project-x-stage-console`
