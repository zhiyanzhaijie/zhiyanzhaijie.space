> 本文是Transformer架构梳理的第一篇文章。  
> 内容为经典Encoder-Decoder架构的可视化拆解，它本身只是我的个人笔记

## Transformer是一台翻译机

Transformer架构登场于Google的Attention is all you need论文里，是Machine Learning领域的一个革新点，它的创作初衷是更高效地解决机器翻译Seq2seq(Sequence to Sequence)问题。

所以，我将Transformer视作翻译机。
请看下面的迷你架构图：

<Mermaid>
flowchart LR
  Source["源句"] --> Encoders["Encoders"]
  Encoders --> Memory["语义上下文"]
  Memory --> Decoders["Decoders"]
  Decoders --> Target["目标句"]

</Mermaid>

图中，原先的句子先是被encoder处理成中间态的语义上下文，decoder再据上下文，逐字（**自回归**）地生成一个结果句。

整个过程是一个翻译行为。要理解经典Transformer之前，代表着我们需要理解翻译行为本身。

--- 
## 翻译的难点

物理世界里，翻译行为可以分为两步：

1. 译者收到原信息，结合自身知识**理解**上下文，并暂存消化后上下文
2. 译者根据上下文陈述

Transformer也是这样子做的。Encoder对应第一步，Decoder对应第二步。
不过, 机器过程没有人类逻辑复杂，我们早已习惯翻译，胆子大点，来理一理机器过程吧。

看起来只有两个步骤，而难点在于——要怎么实现呢？
## Encoder - 理解的实现

Encoder是这样做的。例，源句为$I love your dog$:

### 嵌入(Embedding)

嵌入分为几步：
 1. Tokenization
 2. Embedding
 3. Position Embedding

<Mermaid>
flowchart LR
  A["原句: I love your dog"] --> B["Tokenization: X=(x_1,x_2,...,x_n)"]
  B --> C["Embedding: $$x_i \in \mathbb{R}^{d_{\text{model}}}$$"]
  C --> D["Position Embedding: $$x_i + p_i$$"]
  D --> E["输入矩阵: $$X \in \mathbb{R}^{n \times d_{\text{model}}}$$"]
</Mermaid>


原 Sequence（语句）经过Tokenization被拆解为多个片段，成为Token（词元）序列(不同算法拆分结果不一，但为了保持示例简洁，下文的所有示例token都采用整词拆解形式)。设拆解个数为 $n$，表示为：

$$
X=(x_1,x_2,\dots,x_n)
$$

上述过程中，$I love your dog$首先被拆解为$（I，love，your，dog）$

然后，对每个token进行数学建模。 
比如第一个token 可以写成：
$$
x_1=[0.12,-0.31,0.77,\dots]\in \mathbb{R}^{d_{\text{model}}}
$$

把整句堆起来后，就得到输入矩阵：
$$
X\in\mathbb{R}^{n\times d_{\text{model}}}
$$

假设我们的模型训练维度为6维，每个 token 就被转化成 6 维向量表示：

$$
X =
\begin{bmatrix}
0.12 & -0.5 & 1.0 & 0.0 & 0.45 & -0.1 \\
0.0 & 0.88 & 0.1 & 1.2 & -0.3 & 0.5 \\
-0.2 & 0.1 & 0.95 & -0.1 & 1.1 & 0.0 \\
0.05 & -0.1 & 0.0 & 0.8 & 0.3 & 1.5
\end{bmatrix}
\begin{matrix}
\text{(I)}\\
\text{(love)}\\
\text{(your)}\\
\text{(dog)}
\end{matrix}
$$

几何视角上，这就是 4 个漂浮在 6 维空间里的点。它们现在是“孤立”的, 只有词义，没有句义：$I$ 只是I，$love$ 只是love，$dog$ 也还只是词典意义上的dog。
如何让这4个词互相分享信息，各自影响(表现为向量修正), 这是下一步Attention要做的事情。

#### Position Embedding 位置编码

前面只完成了原词义向量化任务，第三步还需要在结果向量中携带每个token位于源句的位置信息。
这里边会用到位置编码，有了位置信息，才能确保语句上下文自注意力有效。这里不展开。


### 1) Self-Attention： 自注意力机制

Attention 解决的是：让表示矩阵中的每一行都完成它自身的语义补充。流程如下：

<Mermaid>
flowchart LR
  X["输入表示矩阵 $$X$$"] --> QKV["生成 $$Q,K,V$$ 矩阵"]
  QKV --> Score["打分矩阵 $$S=QK^\top$$"]
  Score --> Scale["缩放 $$S/\sqrt{d_k}$$"]
  Scale --> Softmax["注意力矩阵 $$A=\operatorname{softmax}(S/\sqrt{d_k})$$"]
  Softmax --> Mix["输出矩阵 $$O=AV$$"]
  Mix --> Out["语义补充后的表示矩阵"]
</Mermaid>

数学表示见：

$$
\text{Attention}(Q,K,V)=\text{softmax}\left(\frac{QK^\top}{\sqrt{d_k}}\right)V
$$

式子中参数虽多，但从函数视角看是非常清晰的。

我们逐个展开。

> Attention过程涉及大量的矩阵运算，如果你对此一无所知，我推荐你先查看[矩阵的几何直观](/blog/matrix_geometric_intuition).

#### Q、K、V

我们的输入是$X$，而式子中，被Attention函数包裹的是$QKV$, 应该好奇，$X$和$QKV$的关系是什么？

实际上，$QKV$代表的, 正是模型训练后习得的**权重经验**从三个角度对源句矩阵$X$进行投影所得的具有特殊目的矩阵——`权重矩阵`。

权重矩阵通常称为 $W$，像一份被反复打磨过的`"舞台剧本"`:
> 句子X是一出戏剧，每个 token 都是戏剧中的一个角色，角色们一开始只知道自己的角色内容，整出戏是否完美（语义是否完美），要靠它和其余演员充分**对戏**(Attention)。

到了 Self-Attention 这里，这份“剧本W”分成三部分：$W^Q,W^K,W^V$。它们不是输入句子的一部分，而是模型的参数。输入表示矩阵 $X$ 每进入一层，都会按这三组参数生成对应的 $Q,K,V$ 矩阵。

> 模型之初，$W$内部只是一个装着一堆随机数的大矩阵。这个初始矩阵不太可能实现良好的最终seq2seq输出，但经过训练，初始的权重随机数将不断优化而得到更加有效的新矩阵。


实现上，先把当前表示矩阵 $X$ 投影成三组矩阵：

$$
Q=XW^Q,\quad K=XW^K,\quad V=XW^V
$$

如果只看矩阵中的第 $i$ 行，它们代表着：

- $q_i$：这一行表示“我在找什么”（和我对戏的有哪些角色)
- $k_i$：这一行表示“我如何被匹配”（我的角色是什么）
- $v_i$：这一行表示“我能贡献什么内容”（我的戏份是什么） 


$X$一次性和$W$发生运算。它等价于每一行分别生成自己的 $q_i,k_i,v_i$：

$$
q_i=x_iW^Q,\quad k_i=x_iW^K,\quad v_i=x_iW^V
$$

> 注：权重矩阵$W$虽然分为$W^Q$,$W^K$,$W^V$三个部分，但在实际工程中表现成一个大矩阵，在编码中通过切片进行运算获取对应的QKV矩阵。
> 而之所以这样做，是为了充分发挥GPU大矩阵乘法的优势


#### Q、K、V 生成例子

假设当前输入矩阵 $X$ 是 $4\times 6$, 同时拟三尺寸先沟通的$W^Q$,$W^K$,$W^V$。注意：这些数值只是为了演示矩阵链路，不代表真实模型训练出来的参数。

> 权重矩阵的训练时机并在谷歌Attention is all you need论文的某个环节里，它的内容足够单开文章，这里暂不进行展开。

三组权重矩阵可以一起写成：

$$
W^Q=
\begin{bmatrix}
1 & 0 & 0 \\
0 & 1 & 0 \\
0.5 & 0.5 & 0 \\
0 & 0 & 1 \\
0 & 0.5 & 0.5 \\
1 & 0 & 1
\end{bmatrix}
\in\mathbb{R}^{6\times 3}
$$

$$
W^K=
\begin{bmatrix}
0.6 & 0.1 & 0.0 \\
0.0 & 0.8 & 0.2 \\
0.4 & 0.3 & 0.1 \\
0.1 & 0.0 & 0.9 \\
0.0 & 0.7 & 0.4 \\
0.5 & 0.0 & 0.8
\end{bmatrix}
\in\mathbb{R}^{6\times 3}
$$

$$
W^V=
\begin{bmatrix}
0.9 & 0.0 & 0.1 & 0.0 & 0.2 & 0.0 \\
0.0 & 0.8 & 0.0 & 0.2 & 0.0 & 0.1 \\
0.2 & 0.1 & 0.9 & 0.0 & 0.3 & 0.0 \\
0.0 & 0.2 & 0.0 & 0.8 & 0.1 & 0.4 \\
0.1 & 0.0 & 0.3 & 0.1 & 0.9 & 0.0 \\
0.0 & 0.1 & 0.0 & 0.4 & 0.0 & 0.9
\end{bmatrix}
\in\mathbb{R}^{6\times 6}
$$

于是整句一次性相乘，得到三组结果矩阵：

$$
Q=XW^Q=
\begin{bmatrix}
0.520 & 0.225 & 0.125 \\
0.550 & 0.780 & 1.550 \\
0.275 & 1.125 & 0.450 \\
1.550 & 0.050 & 2.450
\end{bmatrix}
\begin{matrix}
\text{(I)}\\
\text{(love)}\\
\text{(your)}\\
\text{(dog)}
\end{matrix}
$$

$$
K=XW^K=
\begin{bmatrix}
0.422 & 0.227 & 0.100 \\
0.410 & 0.524 & 1.546 \\
0.250 & 1.115 & 0.465 \\
0.860 & 0.135 & 2.020
\end{bmatrix}
\begin{matrix}
\text{(I)}\\
\text{(love)}\\
\text{(your)}\\
\text{(dog)}
\end{matrix}
$$

$$
V=XW^V=
\begin{bmatrix}
0.353 & -0.310 & 1.047 & -0.095 & 0.729 & -0.140 \\
-0.010 & 1.004 & 0.000 & 1.306 & -0.120 & 1.018 \\
0.120 & 0.155 & 1.165 & 0.050 & 1.225 & -0.030 \\
0.075 & 0.230 & 0.095 & 1.250 & 0.360 & 1.660
\end{bmatrix}
\begin{matrix}
\text{(I)}\\
\text{(love)}\\
\text{(your)}\\
\text{(dog)}
\end{matrix}
$$

同一个 $X$被投影到了三种不同的权重矩阵里, 而这三个矩阵构成Attention的逻辑核心。

> 注1: $Q,K,V$ 都是整句话共同组成的矩阵，而不是 $1\times n$ 的临时向量。后面之所以能抽出 $q_{love}$ 或 $k_I$，只是因为它们分别是 $Q,K$ 矩阵中的某一行。

> 注2: 这里让 $W^V$ 输出 6 维，是为了让softmax结果可以直接回到主模型维度d_model；
真实工程中也常见先输出较短的 $d_v$，最后在额外用 $W_O$ 投回 $d_{\text{model}}$。


####  Q·K^T （点积）


获得三个矩阵后，Attention（拍戏）过程的第一步是$Q$$K$矩阵运算，但特别说明，形式为$Q \dot K^T$，对$K$矩阵进行倒置, 因为$6 X 3$和$3 X 6$才是合法的矩阵运算, 结果我们暂称$S$。公式为：

$$
S=QK^\top\in\mathbb{R}^{n\times n}
$$

<Mermaid>
flowchart LR
  Qm["$$Q\in\mathbb{R}^{n\times d_k}$$"] --> M["矩阵乘法"]
  Kt["$$K^\top\in\mathbb{R}^{d_k\times n}$$"] --> M
  M --> S["$$S=QK^\top\in\mathbb{R}^{n\times n}$$"]
  S --> Cell["S_ij: 第 i 行看第 j 行的分数"]
</Mermaid>

这里第 $i$ 行表示:
  - “第 $i$ 个 token 表示行在看谁”，
  - 第 $j$ 列表示“它看第 $j$ 个 token 表示行的分数”  

这里拿 $love$ 看 $I$ 做一次完整展开。为了避免跳读，先把完整 $Q,K$ 再摆出来：

$$
Q=
\begin{bmatrix}
0.520 & 0.225 & 0.125 \\
\mathbf{0.550} & \mathbf{0.780} & \mathbf{1.550} \\
0.275 & 1.125 & 0.450 \\
1.550 & 0.050 & 2.450
\end{bmatrix}
\begin{matrix}
\text{(I)}\\
\text{(love)}\\
\text{(your)}\\
\text{(dog)}
\end{matrix}
$$

$$
K=
\begin{bmatrix}
\mathbf{0.422} & \mathbf{0.227} & \mathbf{0.100} \\
0.410 & 0.524 & 1.546 \\
0.250 & 1.115 & 0.465 \\
0.860 & 0.135 & 2.020
\end{bmatrix}
\begin{matrix}
\text{(I)}\\
\text{(love)}\\
\text{(your)}\\
\text{(dog)}
\end{matrix}
$$

于是：

$$
q_{love}=Q_{\text{love},:}=
\begin{bmatrix}
0.550 & 0.780 & 1.550
\end{bmatrix}
$$

$$
k_I=K_{I,:}=
\begin{bmatrix}
0.422 & 0.227 & 0.100
\end{bmatrix}
$$

它们的关联度(需要对戏的程度)得分为：
$$
\begin{align}
\text{Score}_{love,I}
&= q_{love}k_I^\top \\
&=
\begin{bmatrix}
0.55 & 0.78 & 1.55
\end{bmatrix}
\begin{bmatrix}
0.422\\
0.227\\
0.100
\end{bmatrix} \\
&=
0.55\times0.422+0.78\times0.227+1.55\times0.100 \\
&= 0.564
\end{align}
$$

这里发生的是两个向量的点积，结果形式为数值。  
该数值反映的是对词$love$而言词$i$的相关性，它意味着在最终翻译$love$时，应该从$i$中获取补充的程度。
用拍戏比喻，它所代表的是为了完美演出该剧本，角色$love$的戏份里应与角色$i$的对戏占据多少：


由于我们的模型d_model维度很小，这个点积结果可以直观成下面的雷达图, 其中，重合度反映相关度。

<Mermaid>
radar-beta
  title "q_love 与 k_I 的三维雷达重合（Score≈0.564）"
  axis d1["维度 1"], d2["维度 2"], d3["维度 3"]
  curve q["q_love"]{0.550, 0.780, 1.550}
  curve k["k_I"]{0.422, 0.227, 0.100}
  max 1.6
  min 0
</Mermaid>

不要忘了，我们只是取了其中一对token进行了举例，而实际运算是整个矩阵单次运算。  
这是Attention的第一步，该阶段的产物，可以说是**每个角色都知道了剧本中和其它角色的对戏关系**。


#### Scaled - $\sqrt{d_k}$与$softmax$

得到 scores 后，每一行会先除以 $\sqrt{d_k}$ 做缩放，再过$softmax$：

不过，为了更好讲明白这两个内容，需要先解释$softmax$的工作:

经过softmax，原先相似度结果矩阵的每个值$s_ij$将从score（相似得分）变成score_rate（得分占比），含义在于对$Token_i$而言，其全部的注意力中所拿出来关注$Token_j$的注意力占比。


其经典的数学实现为:

$$
\operatorname{softmax}(x_i)=
\frac{e^{x_i}}{\sum_{j=1}^{n} e^{x_j}},
\quad i=1,2,\dots,n
$$

此时，对矩阵里任意$Token_i$（x_i）所持行向量而言，向量每个值变成占比，得分越高占比越大，总和为1。可以将该过程视为**注意力分配**。


在$softmax$公式中，可见其依赖$mathrm{e}$，这意味着，如果存在极大极小的两极情况，会导致全部比重聚焦在极大区域。

而$\sqrt{d_k}$就是来解决这个问题的，它位置称为缩放因子。缩放因子的用意在于对$Q \cdot K^T$做范围控制，控制结果矩阵的方差在一个区间避免两极情况。

> 缩放因子为什么是$\sqrt{d_k}$而不是别的？Transformer原论文中之所以出现这个值源自一个假设，既矩阵$Q,K$内的向量$q,k$满足均值为0,方差为1的特征，它们点击后结果方差为$d_k$。$softmax$要处理的数据，理想方差为1，而能够使得缩放后结果方差为1的缩放因子值便是$\sqrt{d_k}$。  
> 而这个假设源自机器学习行业的常规认知，它并非唯一解。如果感兴趣，可以查看下面两篇文章  
> [苏剑林-浅谈Transformer的初始化、参数化与标准化](https://kexue.fm/archives/8620#NTK%E5%8F%82%E6%95%B0%E5%8C%96)  
> [苏剑林-从熵不变性看Attention的Scale操作](https://kexue.fm/archives/8823#%E7%86%B5%E4%B8%8D%E5%8F%98%E6%80%A7)  


#### V

通过上一步我们得到了一个比例矩阵，它代表着每个$Token$都知道在最终原句语义生成时，向其它token获取语义补充的权重。
拍戏比喻下，这意味着每个角色都知道在权重矩阵$W$剧本下，为了完美演绎，角色应该和哪些其它同事进行不同程度的对戏。

然而光是知道对戏还不够，还需要知道的是——对方的戏份。而这便是权重矩阵$V$的位置。

整体过程为:

<Mermaid>
flowchart LR
  Scores["第 i 行 scores: s_i"] --> Softmax2["softmax"]
  Softmax2 --> A["注意力比例 $$a_i$$"]
  Vmat["Value 矩阵 $$V$$"] --> Mix2["按比例加权求和"]
  A --> Mix2
  Mix2 --> O["输出行 $$o_i$$"]
</Mermaid>

这就是输出矩阵 $O$ 的第 $i$ 行，也就是第 $i$ 个 token 表示行吸收全场信息后的新表示。

比如某一层里，$love$ 对全句的注意力比例可能是：

$$
a_{love}=[0.30,0.45,0.10,0.15]
$$

那么它的新表示就是：

$$
o_{love}=0.30v_I+0.45v_{love}+0.10v_{your}+0.15v_{dog}
$$

这一步就是“向量修正”。原本 $love$ 所在的坐标点，会吸收 $I,your,dog$ 等 token 的信息。于是它不再只是词典里那个孤立的动词，而变成了“由 I 发出、指向某个对象的 love”。
其中，$I$参与了$love$的词义补正，这个过程有术语叫"Attend To"。
这也符合我们的拍戏比喻，角色$I$"Attend To"了角色$love$的戏份。


#### 归纳


总览全程，Self-Attention过程分为：

1. 用 $W^Q,W^K,W^V$ 把表示矩阵投影成三种视角
2. 用 $QK^\top$ 得到 token 表示行之间的对戏强度
3. 用 softmax 把强度变成关注比例
4. 用这些比例混合 $V$，完成每个 token 的向量修正

也就是说，每一层 Attention 都在让词向量发生一次空间位移。层数堆起来后，每个 token 的向量里都会逐渐带上全句的影子。
读者可以在此处再次脑补一下我们的拍戏比喻。

#### 多头注意力

Multi-Head Attention并不复杂。它只是同一群演员换了排练方式，从一种角度更换成多种角度(Head)：
- 有的头关注语法依赖，
- 有的头关注指代关系，
- 有的头关注语义搭配。

如果输入仍记作 $X\in\mathbb{R}^{n\times d_{\text{model}}}$，第 $i$ 个 head 会拥有自己的一组投影矩阵：

$$
\begin{align}
Q_i&=XW_i^Q,\quad W_i^Q\in\mathbb{R}^{d_{\text{model}}\times d_k}\\
K_i&=XW_i^K,\quad W_i^K\in\mathbb{R}^{d_{\text{model}}\times d_k}\\
V_i&=XW_i^V,\quad W_i^V\in\mathbb{R}^{d_{\text{model}}\times d_v}
\end{align}
$$

所以第 $i$ 个 head 的输出就是：

$$
\begin{align}
\text{head}_i
&=\text{Attention}(Q_i,K_i,V_i)\\
&=\text{Attention}(XW_i^Q,XW_i^K,XW_i^V)
\end{align}
$$

多个 head 的结果拼接后，再通过 $W^O$ 投回主模型维度，继续交给后续层处理：

$$
\begin{align}
\text{MultiHead}(Q,K,V)
&=\operatorname{Concat}(\text{head}_1,\text{head}_2,\dots,\text{head}_h)W^O\\
W^O&\in\mathbb{R}^{hd_v\times d_{\text{model}}}
\end{align}
$$

其中，$d_k=d_v=d_{\text{model}}/h$，每个 head 只看一小段子空间；$h$ 个 head 拼接起来后，维度又回到 $d_{\text{model}}$。
读者可以自行推演$Q,K,V$矩阵尺寸与模型维度$d_model$的关系。


拿我们上面的例子来看，过程图为：

<Mermaid>
flowchart LR
  Xm["输入 $$X$$"] --> Head1["Head 1: 语法依赖"]
  Xm --> Head2["Head 2: 指代关系"]
  Xm --> Head3["Head 3: 语义搭配"]
  Head1 --> Cat["Concat 多头结果"]
  Head2 --> Cat
  Head3 --> Cat
  Cat --> WO["乘 $$W_O$$ 投回主空间"]
  WO --> Out2["Multi-Head 输出"]
</Mermaid>

> 注意：
> 多头意味着Attention所依赖的权重矩阵应有多个, 并且每个头对应的$W_i$都是独立的权重参数。


### 2) Add & Norm

Self-Attention 已经把每个 token 的向量修正过一次。但直接把这个修正结果传下去，深层训练很容易出问题：数值可能越漂越大，原始词义也可能慢慢丢失。

所以 Transformer结构中每一层都加入一个稳定组件：

$$
X'=\text{LayerNorm}(X+\text{SelfAttention}(X))
$$

它分为两步：

- **Add**：把 Self-Attention 的输出和原始输入 $X$ 相加，保留原始信号。
- **LayerNorm**：对相加后的结果做归一化，把数值压回稳定范围。

<Mermaid>
flowchart LR
  X0["输入 X"] --> Plus["残差相加 +"]
  SA["Self-Attention(X)"] --> Plus
  Plus --> LN["LayerNorm"]
  LN --> Xp["X'"]
</Mermaid>

FFN 之后还会再来一次：

$$
X_{next}=\text{LayerNorm}(X'+\text{FFN}(X'))
$$

也就是说，**Attention 和 FFN 各自前后都有一次 Add & Norm**。它们共同保证信息的稳定传递。

#### Add - 残差连接

假设经过 Self-Attention 后，token `love` 的向量从：

$$
x_{love}=[0.4,0.5,0.6]
$$

被修正成：

$$
\text{SelfAttention}(x_{love})=[2.1,-1.0,0.3]
$$

如果没有残差连接，下一层收到的就是 `[2.1, -1.0, 0.3]`。这个向量已经和 `love` 的原始向量 `[0.4, 0.5, 0.6]` 没多大关系了。如果每层都这样改，6 层(论文中Encoder Layer的层数)之后，模型可能完全忘记 `love` 原本是什么意思。

加上残差连接后：
$$
\begin{align}
x_{love}'
&= x_{love} + \text{SelfAttention}(x_{love}) \\
&= [0.4,0.5,0.6] + [2.1,-1.0,0.3] \\
&= [2.5,-0.5,0.9]
\end{align}
$$

虽然数值变了，但 `[2.5, -0.5, 0.9]` 这个向量仍然**包含原始 `[0.4, 0.5, 0.6]` 的成分**。原始信号没有被丢弃，而是作为“底子”保留了下来。

用拍戏比喻来说：**$Attention$ 让每个演员重新理解了自己的戏份，但$Add$让演员不会忘记自己“原本是谁”**。

除了保留信息，Add 还有另一个重要作用：
深度学习算法中是通过**反向传播机制**来调整模型参数实现训练的，直接依赖于梯度，通过梯度指导模型参数优化。
梯度的计算是从深层(靠近输出)向浅层(靠近输入)不断链式累乘运算，如果不做任何处理，会出现梯度在前面就趋向于零（梯度消失），这样，浅层的参数将无法得到有效训练, 而通过$Add$，在运算时就能够确保梯度稳定到达浅层。 

核心就在于残差连接的数学形式：**y = x + f(x)** 的导数里天然带一个 +1（单位矩阵）。展开一下:

1. 没有 Add 时

假设网络的一层是：

$$
y = f(x)
$$

其中 $f$ 可能包含 Attention、FFN、LayerNorm 等。

反向传播求梯度：

$$
\frac{\partial L}{\partial x} = \frac{\partial L}{\partial y} \cdot \frac{\partial y}{\partial x}
= \frac{\partial L}{\partial y} \cdot f'(x)
$$

如果网络很深(仍以6层为例)：

$$
x_6 = f_6(f_5(f_4(f_3(f_2(f_1(x))))))
$$

那么梯度从 Loss 传回 $x_1$ 时要连乘：

$$
\frac{\partial L}{\partial x_1}
= \frac{\partial L}{\partial x_6}
\cdot f_6'(x_5)
\cdot f_5'(x_4)
\cdot f_4'(x_3)
\cdot f_3'(x_2)
\cdot f_2'(x_1)
\cdot f_1'(x)
$$

如果每个 $f_i'(x)$ 的范数都小于 1（比如 0.6），那么 6 层乘起来：

$$
0.6^6 \approx 0.047
$$

越往浅层，梯度越小，参数更新越慢，这就是梯度消失。


2. 有 Add 时

残差连接变成：

$$
y = x + f(x)
$$

导数变成：

$$
\frac{\partial y}{\partial x} = I + f'(x)
$$

其中 $I$ 是单位矩阵。

反向传播：

$$
\frac{\partial L}{\partial x}
= \frac{\partial L}{\partial y} \cdot \frac{\partial y}{\partial x}
= \frac{\partial L}{\partial y} \cdot (I + f'(x))
$$

即使 $f'(x)$ 很小，梯度里仍然保留了一个完整的 $\frac{\partial L}{\partial y}$ 项。


#### Layer Normalization 标准/归一化

如果没有 Norm，残差相加后的数值会一层一层累加。例如，假设每层的修正量都差不多：

$$
[0.4,0.5,0.6] + [2.1,-1.0,0.3] = [2.5,-0.5,0.9]
$$

第二层继续加：

$$
[2.5,-0.5,0.9] + [2.1,-1.0,0.3] = [4.6,-1.5,1.2]
$$

到第六层时，向量可能已经变成：

$$
[13.0,-5.5,2.4]
$$

数值越来越大，分布也越来越不稳定。后面的层拿到这种输入，训练会变得非常困难。

那么，LayerNorm 的作用来了：对每个 token 向量做归一化，把**均值压到 0、方差压到 1**，再通过可学习的 $\gamma$ 和 $\beta$ 微调：

$$
\text{LayerNorm}(x)=\gamma\cdot\frac{x-\mu}{\sqrt{\sigma^2+\varepsilon}}+\beta
$$

> 在原论文的 Transformer 里，Add 和 Norm 是 Post-Norm 形式：先加，再归一化。现代很多大模型（如 GPT、LLaMA）则采用 Pre-Norm：
>> $$
>> X_{next}=X+\text{FFN}(\text{LayerNorm}(X))
>> $$


### 3) FFN：MLP is still what you need

Attention is all you need 这篇论文的名字只强调了 Attention，但 Transformer 的真正核心其实是 **Attention + FFN**。

- **Attention** 解决“token 之间如何交换信息”。
- **FFN** 解决“每个 token 拿到信息之后，如何在自己内部重新加工”。

如果把每个 token 看作一个演员，那么 Attention 是让演员之间对戏，FFN 则是让演员在对完戏后，自己消化自己的剧本进行后续发挥。

FFN 的公式是：

$$
\text{FFN}(x)=\max(0,xW_1+b_1)W_2+b_2
$$

其中 $\max(0, \dots)$ 是 ReLU 激活，现代模型也常用 GELU。

<Mermaid>
flowchart LR
  Xi["单个位置 x_i'"] --> Up["W1 升维"]
  Up --> Act["ReLU / GELU 非线性"]
  Act --> Down["W2 降回 d_model"]
  Down --> Residual["与 x_i' 残差相加"]
  Residual --> LN2["LayerNorm"]
  LN2 --> Next["下一层输入"]
</Mermaid>

#### Attention + FFN？

Attention 的输出本质上是：

$$
\text{Attention Output} = \sum_j a_{ij} v_j
$$

也就是说，每个位置的新向量是其他位置 Value 向量的**加权平均**。它仍然是输入空间里的线性组合，只是权重不同。

如果只有 Attention，不管堆多少层，模型学到的东西都会受到限制：**它只能重新分配已有的信息，不能创造新的、更抽象的特征**。

FFN 的作用就是给模型加入**非线性变换能力**，让模型掌握类如“如果存在某特征，就增强；如果不存在，就抑制”的能力。

> FFN是单层共用的：
>> 对第 $i$ 个 token：
>> 
>> $$
>> x_i' \rightarrow \text{FFN}(x_i')
>> $$
>> 对第 $j$ 个 token：
>> 
>> $$
>> x_j' \rightarrow \text{FFN}(x_j')
>> $$
>> 两者使用同一套参数 $W_1, b_1, W_2, b_2$。

#### 升维、激活、降维

FFN 过程为`升维 -> 激活 -> 降维`：

$$
d_{\text{model}} \rightarrow d_{\text{ff}} \rightarrow d_{\text{model}}
$$

原论文里 $d_{\text{ff}} = 4 \times d_{\text{model}}$。比如 $d_{\text{model}} = 512$ 时，FFN 中间维度就是 2048。

这样的好处是：

1. **升维**：把语义展开到更高维空间，让原本纠缠在一起的特征更容易分开处理。
2. **激活**：用非线性函数把线性不可分的问题变成可分的问题。
3. **降维**：把结果压回 $d_{\text{model}}$，保证下一层 Encoder 能继续接下去。

#### Key-Value 记忆

除了非线性变换，FFN 还承担了一个重要角色：**存储知识**。

近年来很多可解释性研究认为，FFN 可以被理解为一种 **Key-Value 记忆**。

- $W_1$ 负责把输入向量匹配到某些“键”（特定神经元）。
- $W_2$ 负责输出这些神经元对应的“值”（语义补充）。

每个隐藏层神经元就像一个知识条目。当输入向量匹配某个条目时，这个神经元被激活，然后通过 $W_2$ 把相关知识写进残差流。

比如输入中出现 `Michael Jordan`：

1. 它的向量经过 $W_1$ 投影。
2. 某些神经元被激活，这些神经元在训练过程中学会了“Michael Jordan 相关模式”。
3. 被激活的神经元通过 $W_2$ 输出一段方向，对应“篮球运动员”、“NBA”、“芝加哥公牛”等语义。
4. 这段输出通过残差连接加到原始向量上。

所以，Michael Jordan 是篮球运动员这个信息，**不在输入 token 里，而在 FFN 的权重里**。


因为FFN层承载如此多的信息，所以一个 Transformer 模型的主要参数不在 Attention，而在 FFN：

$$
\text{FFN 每层参数量} \approx 2 \times d_{\text{model}} \times d_{\text{ff}}
$$

如果 $d_{\text{ff}} = 4 \times d_{\text{model}}$，那么 FFN 参数就是 Attention 参数的数倍。

所以“把模型做大”很大程度上就是扩大 FFN 的隐藏层维度。FFN 越大，能存储和处理的知识条目就越多，模型能力通常也越强。


> 和 Attention 后面一样，FFN 输出之后也要做一次$Add Norm$：
>> $$
>> X_{next}=\text{LayerNorm}(X'+\text{FFN}(X'))
>> $$

到此，一个层内的流程跑完。输出结果将传递到下一层，如果已经是Encoder堆的最后一层，Encoder 输出最终上下文表示：
$$
Z=(z_1,z_2,\dots,z_n)
$$

姑且把 $Z$ 叫作 memory（Attention is all you need 论文本身没有命名它，但多数实现代码中叫 memory），作为模型“已理解后的语义记忆”，随后交给 Decoder 去完成陈述与生成。


## Decoder：陈述

Encoder 把源句读成 memory $Z$ 后，Decoder 负责根据这份记忆，逐词生成目标句。

它不是一次性输出整句，而是**自回归**地一个 token 一个 token 生成：

$$
\text{<BOS>} \rightarrow \text{我} \rightarrow \text{爱} \rightarrow \text{你的} \rightarrow \text{狗}
$$

> `<BOS>` 是 begin of sentence，  
> `<EOS>` 是 end of sentence。  
> 生成 `<EOS>` 时，代表整句翻译完成。

Decoder 的输入不是源句$X$，而是目标端已经生成的部分，经过 Embedding 和 Position Embedding 后得到矩阵 $Y$。它的处理
流程如下：

<Mermaid>
flowchart LR
  Tgt["已生成目标序列"] --> Emb["目标端 Embedding"]
  Emb --> Masked["Masked Self-Attention"]
  Masked --> Cross["Cross-Attention"]
  Memory["Encoder memory Z"] --> Cross
  Cross --> FFN["FFN"]
  FFN --> Linear["Linear + Softmax"]
  Linear --> Next["下一个 token"]
</Mermaid>

Decoder中特别地，有两个Attention层。

### Masked Self-Attention

Decoder 的第一层 Attention 和 Encoder 很像，但多了一个 mask：**生成第 $t$ 个词时，不能看第 $t+1$ 个及以后的位置。否则训练时它会直接抄答案。**

> Decoder有两层Attention，它们各自拥有不同的权重矩阵$W$。

公式上只在 softmax 前加一个 mask 矩阵 $M$：

$$
\text{MaskedAttention}(Q_Y,K_Y,V_Y)=
\text{softmax}\left(\frac{Q_YK_Y^\top+M}{\sqrt{d_k}}\right)V_Y
$$

允许看的位置填 $0$，未来位置填 $-\infty$。这样 softmax 后，未来位置的概率就变成 0。

> 目标端有 4 个位置时，mask长这样：
> $$
> M=\begin{bmatrix}
> 0 & -\infty & -\infty & -\infty\\
> 0 & 0 & -\infty & -\infty\\
> 0 & 0 & 0 & -\infty\\
> 0 & 0 & 0 & 0
> \end{bmatrix}
> $$
> 通过置入极小值，屏蔽后续值干扰


### Cross-Attention

Cross-Attention 的 Cross 来自于它跨越两个序列：Decoder 的当前序列和 Encoder 的源序列。

它和Self-Attention的关键区别：$Q$ 通过Masked传入，$K,V$ 来自 Encoder memory：

$$
Q=Y'W^Q,\quad K=ZW^K,\quad V=ZW^V
$$

> 这里 $Y'$ 是 Masked Self-Attention 的输出，$Z$ 是 Encoder 最终输出的 memory。

含义是：Decoder 当前位置通过 Query 去问，“原文里哪些位置和我现在要翻译的内容相关？”然后取走对应 Value 里的语义。

比如生成“狗”时，Decoder 的 Cross-Attention 会强烈关注 Encoder memory 中对应 `dog` 的那一行；生成“你的”时，则更关注 `your`。这就是翻译中的“对齐”。

### 例 - 两个时刻


我们用源句 `I love your dog`，挑选两个自回归时刻做演示：

$$
\text{<BOS>} \rightarrow \text{我} \rightarrow \text{爱} \rightarrow \text{你的} \rightarrow \text{狗}
$$

**时刻一：生成“我”**

Decoder 输入只有 `<BOS>`。经过 Masked Self-Attention 后，它知道“句子刚开始”。Cross-Attention 的 Query 带着这个状态去问原文 memory：

> “源句开头是什么？该先翻译什么？”

原文 memory 中对应 `I` 的响应最强，所以 Decoder 输出 “我”。

**时刻二：生成“狗”**

此前 Decoder 已经生成 `<BOS> 我 爱 你的`。Masked Self-Attention 看到这段前缀，形成状态 $Y'$：接下来需要接一个名词。Cross-Attention 的 Query 带着这个状态去问原文：

> “我已经说了‘我 爱 你的’，接下来需要名词，原文里哪个词该被翻译？”

原文 memory 中对应 `dog` 的响应最强，所以 Decoder 输出 “狗”。

每个时刻生成的 token 都会被追加回目标序列，继续下一轮。

### Add & Norm

Decoder 层内有两处 Attention，一样的，后面都接 Add & Norm，最后再过 FFN：

> $$
> Y'=\text{LayerNorm}(Y+\text{MaskedSelfAttention}(Y))
> $$
> $$
> Y''=\text{LayerNorm}(Y'+\text{CrossAttention}(Y',Z))
> $$
> $$
> Y_{\text{next}}=\text{LayerNorm}(Y''+\text{FFN}(Y''))
> $$

这里不再展开。

完成这一步，当前层的decoder任务就算完成了，下一步和encoder是一样的。如果当前层不是最后一层，则继续往下一层encoder层传递，反之作为最后一层的输出进入到下一步。

### Linear + Softmax

Decoder 最后一层输出的是一个矩阵。在自回归生成中，我们只需要**最后一个位置**的向量 $o_t$ 来决定下一个词。但 $o_t$ 还在内部语义空间 $\mathbb{R}^{d_{\text{model}}}$ 里，要变成词表上的概率，需要两步。

**第一步：Linear 投影**

通过一个线性层，把 $o_t$ 映射到词表维度：

$$
\text{logits}=o_tW_{\text{vocab}}+b
$$

$$
\mathbb{R}^{d_{\text{model}}}\rightarrow\mathbb{R}^{|\mathcal{V}|}
$$

$W_{\text{vocab}}$ 的每一行对应词表里的一个词。投影后，每个维度代表一个词的“分数”，分数越高，说明这个词越适合作为下一个 token。

> 例如词表里有 5 万个词，logits 就是一个 5 万维向量。假设其中“狗”的分数是 4.2，“猫”是 1.5，“人”是 0.3，模型当前最倾向于生成“狗”。

**第二步：Softmax**

Softmax 把 logits 转成合法的概率分布，所有词的概率之和为 1：

$$
p_t=\text{softmax}(\text{logits})
$$

> 接上面的例子：
> $$
> P(\text{狗})=0.62,\quad P(\text{猫})=0.11,\quad P(\text{人})=0.04
> $$

<Mermaid>
flowchart LR
  Ot["最后位置向量 $$o_t$$"] --> Linear["Linear 投影到词表"]
  Linear --> Logits["logits"]
  Logits --> Softmax["Softmax"]
  Softmax --> Prob["词表概率分布"]
  Prob --> Pick["选择下一个 token"]
</Mermaid>

> 最后，模型根据这个概率分布选择下一个 token, 这里选词方案也存在多种，论文中使用的是`beam search`，这里不展开。
> 选出的 token 会被追加回目标序列，再送入 Decoder 继续下一轮。直到生成 `<EOS>`，整句翻译结束。
> 到这里，经典 Encoder-Decoder Transformer 的主线就闭合了：Encoder 负责把源句读成 memory，Decoder 负责根据 memory 一步步说出目标句。


## 结语

到此，我们已经领略了经典Transformer架构全程。这个典型结构为了解决机器翻译`seq2seq`问题而提出，先入为主地我们将其比作了翻译机，然后从encoder-decoder到内层，逐层拆解，从最初的分词到memory的全句语义构建，再到decoder进行逐字输出译文, 完成地说明了每个层，每个组件的工作机制及用意。

掌握了翻译机结构后，我们才能走向目前炙手可热的大模型知识领域。




