> 本文是Transformer架构梳理的第一篇文章。  
> 内容为经典Encoder-Decoder架构的可视化拆解，它本身只是我的个人笔记

## Transformer是一台翻译机

Transformer架构登场于Google的Attention is all you need论文里，是Machine Learning领域的一个革新点，它的创作初衷是更高效地解决机器翻译Seq2seq(Sequence to Sequence)问题。

所以，我将Transformer视作翻译机。
请看下面的迷你架构图：
<!-- Mermaid - 迷你架构 -->

<Mermaid>
flowchart LR
  Source["源句"] --> Encoders["Encoders"]
  Encoders --> Memory["语义上下文"]
  Memory --> Decoders["Decoders"]
  Decoders --> Target["目标句"]

</Mermaid>

图中，原先的句子先是被encoder处理成中间态的语义上下文，decoder再据上下文，逐字（**自回归**）地生成一个结果句。

整个过程是一个翻译行为。所以，理解经典Transformer之前，得先理解翻译本身。

--- 
## 翻译的难点

在现实物理世界里，我将翻译行为分为两步：

1. 译者收到原信息，结合自身知识**理解**上下文，并暂存上下文
2. 译者根据上下文陈述

实际上，Transformer也是这样子做的。Encoder对应第一步，Decoder对应第二步。
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

几何视角上，这就是 4 个漂浮在 6 维空间里的点。它们现在还比较“孤立”：$I$ 只是I，$love$ 只是love，$dog$ 也还只是词典意义上的dog。
如何让这6各点互相分享信息，各自影响，最终为矩阵增加语义, 这是下一步Attention要做的事情。


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

式子中参数很多，但有理可循。

#### Q、K、V

式子中，被Attention函数包裹的是$Q\quadK\quadV$，而Attention要操作的是输入矩阵$X$, 所以QKV和X必然存在关联。
实际上，QKV代表的, 正是模型使用它在训练中习得的**权重经验$W$**从三个角度对源句矩阵$X$进行投影所得的具有特殊目的矩阵——权重矩阵。

权重矩阵通常称为 $W$，像一份被反复打磨过的舞台剧本；每个 token 都是一个演员，它一开始只知道自己拿到的剧本内容，整出戏是否完美（语义是否完美），要靠它和其余演员充分**对戏**。

到了 Self-Attention 这里，这份“剧本”分成三部分：$W^Q,W^K,W^V$。它们不是输入句子的一部分，而是模型训练后留下来的参数。输入表示矩阵 $X$ 每进入一层，都会按这三组参数生成对应的 $Q,K,V$ 矩阵。

实现上，先把当前表示矩阵 $X$ 投影成三组矩阵：

$$
Q=XW^Q,\quad K=XW^K,\quad V=XW^V
$$

<Mermaid>
flowchart LR
  X0["当前表示矩阵 $$X$$"] --> WQ["乘 $$W^Q$$"]
  X0 --> WK["乘 $$W^K$$"]
  X0 --> WV["乘 $$W^V$$"]
  WQ --> Q0["$$Q$$ 矩阵"]
  WK --> K0["$$K$$ 矩阵"]
  WV --> V0["$$V$$ 矩阵"]
</Mermaid>

如果只看矩阵中的第 $i$ 行，它们分别有这样的角色：

- $q_i$：这一行表示“我在找什么”
- $k_i$：这一行表示“我如何被匹配”
- $v_i$：这一行表示“我能贡献什么内容”

所以 $X$ 一次性乘上 $W^Q,W^K,W^V$，只是并行矩阵写法。它等价于每一行分别生成自己的 $q_i,k_i,v_i$：

$$
q_i=x_iW^Q,\quad k_i=x_iW^K,\quad v_i=x_iW^V
$$

这一步很像“领剧本”：同一个 token 表示行，经过三套不同参数，被投影成三种职能。

#### 第二步：看一个完整的 Q、K、V 生成例子

假设当前输入矩阵 $X$ 是 $4\times 6$。为了让后面的计算更有实感，下面拟三组同尺度的模拟权重。注意：这些数值只是为了演示矩阵链路，不代表真实模型训练出来的参数。

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

这三次乘法的输入都是同一个 $X$，区别只在于右侧乘上的权重矩阵不同。于是同一批 token 表示，被投影到了三种不同的观察空间里：$Q$ 负责“我在找什么”，$K$ 负责“我如何被匹配”，$V$ 负责“如果别人关注我，我能贡献什么内容”。

注意，$Q,K,V$ 都是整句话共同组成的矩阵，而不是 $1\times n$ 的临时向量。后面之所以能抽出 $q_{love}$ 或 $k_I$，只是因为它们分别是 $Q,K$ 矩阵中的某一行。

这里让 $W^V$ 输出 6 维，是为了让 $AV$ 的结果可以直接回到主模型维度；真实 Multi-Head Attention 里也常见先输出较短的 $d_v$，最后再用 $W_O$ 投回 $d_{\text{model}}$。

这也不是简单地“缩短向量”，而是把原始语义放到不同的新坐标系里观察。借用 3Blue1Brown 常讲的线性变换直觉：矩阵的列向量像新的基坐标轴，矩阵乘法就是把一个点投影到这些新轴上。$W^Q$ 让 token 表示变成“搜索信号”，$W^K$ 让 token 表示变成“可被搜索的标签”，$W^V$ 则保留“真正能被取走的内容”。

#### 第三步：用 QK 转置计算表示行之间的打分

生成 $Q,K,V$ 后，不是只算 $q_i k_i^\top$，而是让第 $i$ 行和所有 token 表示行做匹配：

$$
s_i=[q_i k_1^\top,q_i k_2^\top,\dots,q_i k_n^\top]
$$

整体写成矩阵，就是：

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

这里第 $i$ 行表示“第 $i$ 个 token 表示行在看谁”，第 $j$ 列表示“它看第 $j$ 个 token 表示行的分数”。所以 $QK^\top$ 得到的是 token 表示行之间的相似度，而不是特征维度之间的相关性。$K^\top$ 也没有改变 key 的含义，只是把所有 key 摆成适合被 query 批量点积的方向。

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

因为本文采用 token-as-row 写法，所以计算 $love$ 对 $I$ 的注意力打分时，是把 $k_I$ 转置成列向量，放在 $q_{love}$ 的右侧：

$$
\begin{aligned}
\text{Score}_{love,I}
&=q_{love}k_I^\top\\
&=
\begin{bmatrix}
0.55 & 0.78 & 1.55
\end{bmatrix}
\begin{bmatrix}
0.422\\
0.227\\
0.100
\end{bmatrix}\\
&=
\begin{bmatrix}
0.55\times0.422+0.78\times0.227+1.55\times0.100
\end{bmatrix}\\
&=
\begin{bmatrix}
0.564
\end{bmatrix}
\end{aligned}
$$

<Mermaid>
radar-beta
  title "q_love 与 k_I 的三维雷达重合（Score≈0.564）"
  axis d1["维度 1"], d2["维度 2"], d3["维度 3"]
  curve q["q_love"]{0.550, 0.780, 1.550}
  curve k["k_I"]{0.422, 0.227, 0.100}
  max 1.6
  min 0
</Mermaid>

这个 $1\times1$ 的结果，就是 $love$ 这个 query 对 $I$ 这个 key 的原始注意力分数。

几何上，$k_I^\top$ 可以理解成一把由 $I$ 定义出来的线性标尺。$q_{love}$ 乘上 $k_I^\top$，不是把整个空间“倒过来”，而是用这把标尺去测量 $q_{love}$：它在 $I$ 所代表的 key 方向上投影有多强。

如果分数很大，说明在当前语境里，$love$ 这个 token 很需要 attend to $I$；如果分数很小，说明 $I$ 对当前的 $love$ 来说暂时不重要。把这件事对所有 key 都做一遍，就得到 $love$ 那一整行 attention scores。

#### 第四步：用 softmax 变成比例，再混合 Value

得到 scores 后，每一行会先除以 $\sqrt{d_k}$ 做缩放，再过 softmax：

$$
a_i=\text{softmax}\left(\frac{s_i}{\sqrt{d_k}}\right)
$$

最后用这一行比例混合所有 value：

$$
o_i=\sum_{j=1}^{n}a_{ij}v_j
$$

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

这一步就是“向量修正”。原本 $love$ 所在的坐标点，会吸收 $I,your,dog$ 等 token 的信息。于是它不再只是词典里那个孤立的动词，而变成了“由 I 发出、指向某个对象的 love”。这就是 attend to 在数学上的体现：一个 token 因为关注了别的 token，自己的向量位置被重新塑形。

所以 Self-Attention 的核心并不只是“算相关性”，而是：

1. 用 $W^Q,W^K,W^V$ 把表示矩阵投影成三种视角
2. 用 $QK^\top$ 得到 token 表示行之间的对戏强度
3. 用 softmax 把强度变成关注比例
4. 用这些比例混合 $V$，完成每个 token 的向量修正

也就是说，每一层 Attention 都在让词向量发生一次空间位移。层数堆起来后，每个 token 的向量里都会逐渐带上全句的影子。

#### 第五步：多头就是多种视角同时对戏

如果是 Multi-Head Attention，则相当于同一群演员同时从多种角度排练：有的头关注语法依赖，有的头关注指代关系，有的头关注语义搭配。多个 head 的结果拼接后，再通过 $W_O$ 投回主模型空间，继续交给后续层处理。

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

总结，Attention 要做的事，不是让某个 token 单独“理解全句”，而是让每个 token 在全句语境里重新理解自己：我是谁、我和谁有关、我该吸收谁的信息、最后我应该带着怎样的语义继续往后走。


### 2) Add & Norm：稳定训练并保留原信息
注意力输出不会直接替换输入，而是残差相加，再归一化：
$$
X'=\text{LayerNorm}(X+\text{SelfAttention}(X))
$$

<Mermaid>
flowchart LR
  X0["输入 X"] --> Plus["残差相加 +"]
  SA["Self-Attention(X)"] --> Plus
  Plus --> LN["LayerNorm"]
  LN --> Xp["X'"]
</Mermaid>

这样做有两个好处：
- 原始信息不丢（有“捷径”可走）
- 深层网络更稳定（梯度不容易炸或消失）

### 3) FFN：逐位置非线性加工
然后每个位置独立走同一个前馈网络：
$$
\text{FFN}(x)=\max(0,xW_1+b_1)W_2+b_2
$$

再做一次 Add & Norm：
$$
X_{next}=\text{LayerNorm}(X'+\text{FFN}(X'))
$$

<Mermaid>
flowchart LR
  Xi["单个位置 x_i'"] --> Up["W1 升维"]
  Up --> Act["ReLU / GELU 非线性"]
  Act --> Down["W2 降回 d_model"]
  Down --> Residual["与 x_i' 残差相加"]
  Residual --> LN2["LayerNorm"]
  LN2 --> Next["下一层输入"]
</Mermaid>

这表示：先“看全局关系”，再“做本地提纯”。

这里的“逐位置”很重要。Self-Attention 会让 token 之间交换信息，而 FFN 不再让 token 彼此通信。它对每个位置单独处理，但所有位置共享同一套 $W_1,b_1,W_2,b_2$ 参数。

也就是说，对第 $i$ 个 token 来说：

$$
x_i' \rightarrow \text{FFN}(x_i')
$$

对第 $j$ 个 token 来说：

$$
x_j' \rightarrow \text{FFN}(x_j')
$$

两者互不读取，但使用的是同一个函数。这有点像给每个已经融合上下文的 token 都做一次相同规格的“特征重组”。

这个加工通常可以理解为一次“升维—激活—降维”：

1. 先升维：$W_1$ 把向量投影到更宽的隐藏空间。低维里挤在一起的语义，到了高维后更容易被展开。
2. 再激活：ReLU 或 GELU 引入非线性。否则两次矩阵乘法本质上仍然可以合并成一次线性变换，表达能力不够。
3. 最后降维：$W_2$ 把结果压回 $d_{\text{model}}$，保证输出尺寸和输入一致，下一层 Encoder 才能继续接上。

所以，Attention 负责把“别的 token 的信息”写进当前位置，FFN 负责在当前位置内部重新组合这些信息。前者解决“该看谁”，后者解决“看完以后怎么消化”。

当这样的层堆叠多次后，Encoder输出最终上下文表示：
$$
Z=(z_1,z_2,\dots,z_n)
$$

姑且把$Z$叫作memory(Attention is all you need论文上并没有对该产物进行命名，但是多数代码实现上把它命名为memory）,它可以理解为“机器已理解后的语义记忆”，随后交给 Decoder 去完成陈述与生成。

## Decoder - 陈述的实现

Encoder 做完以后，机器已经有了一份对源句的理解，也就是前面说的 memory：

$$
Z=(z_1,z_2,\dots,z_n)
$$

如果 Encoder 像“读懂原句”，那么 Decoder 就像“根据理解往外说”。但它不是一次性把目标句全部吐出来，而是一步一步生成：

$$
\text{<BOS>} \rightarrow \text{I} \rightarrow \text{love} \rightarrow \text{your} \rightarrow \text{dog}
$$

在翻译里，更真实的目标句可能是中文，比如：

$$
\text{<BOS>} \rightarrow \text{我} \rightarrow \text{爱} \rightarrow \text{你的} \rightarrow \text{狗}
$$

这里的 `<BOS>` 表示 begin of sentence，也就是“开始说话”的信号。

Decoder 的整体流程可以先看成这样：

<Mermaid>
flowchart LR
  Tgt["已生成目标序列"] --> Emb["目标端 Embedding"]
  Emb --> Masked["Masked Self-Attention"]
  Masked --> Cross["Cross-Attention"]
  Memory["Encoder memory Z"] --> Cross
  Cross --> FFN["FFN"]
  FFN --> Linear["Linear"]
  Linear --> Softmax["Softmax"]
  Softmax --> Next["下一个 token"]
</Mermaid>

它比 Encoder 多了一件关键事情：Decoder 既要看自己已经说过的话，也要看 Encoder 理解出来的源句上下文。

### 1) 目标端 Embedding：把已经说出的词变成矩阵

Decoder 的输入不是源句，而是目标句中“已经生成出来的部分”。

假设现在模型已经生成了：

$$
Y=(y_1,y_2,\dots,y_m)
$$

经过目标端 Embedding 和 Position Embedding 后，也会得到一个矩阵：

$$
Y\in\mathbb{R}^{m\times d_{\text{model}}}
$$

<Mermaid>
flowchart LR
  A["已生成: BOS, 我, 爱"] --> B["Tokenization"]
  B --> C["目标端 Embedding"]
  C --> D["Position Embedding"]
  D --> E["目标表示矩阵 $$Y$$"]
</Mermaid>

这和 Encoder 的输入矩阵 $X$ 很像，只不过 $X$ 来自源句，$Y$ 来自目标端已经生成的部分。

### 2) Masked Self-Attention：只能看过去，不能偷看未来

Decoder 的第一层 Attention 叫 Masked Self-Attention。

它和 Encoder 的 Self-Attention 很像，也会从 $Y$ 中生成：

$$
Q_Y=YW^Q,\quad K_Y=YW^K,\quad V_Y=YW^V
$$

区别在于：Decoder 生成第 $t$ 个词时，不能提前看到第 $t+1$ 个词。否则训练时它会作弊，直接抄答案。

所以，它需要一个 mask，把未来位置遮住：

<Mermaid>
flowchart LR
  Y0["目标表示矩阵 $$Y$$"] --> QKVY["生成目标端 QKV"]
  QKVY --> ScoreY["打分矩阵"]
  Mask["未来位置 Mask"] --> ScoreY
  ScoreY --> SoftY["Masked Softmax"]
  SoftY --> OY["目标端上下文矩阵"]
</Mermaid>

矩阵上可以理解成：

$$
\begin{aligned}
\text{MaskedAttention}(Q_Y,K_Y,V_Y)
&=
\text{softmax}\left(\frac{Q_YK_Y^\top+M}{\sqrt{d_k}}\right)V_Y
\end{aligned}
$$

其中 $M$ 是 mask 矩阵。允许看的地方是 $0$，不允许看的未来位置是 $-\infty$。这样 softmax 之后，未来位置的概率就会变成 0。

比如目标端已有 4 个位置时，mask 大概长这样：

$$
M=
\begin{bmatrix}
0 & -\infty & -\infty & -\infty\\
0 & 0 & -\infty & -\infty\\
0 & 0 & 0 & -\infty\\
0 & 0 & 0 & 0
\end{bmatrix}
$$

它的含义很直接：

- 第 1 个位置只能看第 1 个位置
- 第 2 个位置可以看第 1、2 个位置
- 第 3 个位置可以看第 1、2、3 个位置
- 越往后，能看的历史越多，但永远不能看未来

<Mermaid>
flowchart TD
  P1["位置 1"] --> A1["可看: 1"]
  P2["位置 2"] --> A2["可看: 1,2"]
  P3["位置 3"] --> A3["可看: 1,2,3"]
  P4["位置 4"] --> A4["可看: 1,2,3,4"]
</Mermaid>

这就是自回归的核心：每一步只能根据过去和现在，猜下一个词。

### 3) Cross-Attention：一边说，一边回看原文

Masked Self-Attention 解决的是“目标句内部的上下文”。比如已经说了“我 爱”，那下一步很可能要说一个宾语。

但翻译不能只看自己说过什么，还要回看原文。这个动作就是 Cross-Attention。

Cross-Attention 的关键在于：$Q$ 来自 Decoder，$K,V$ 来自 Encoder。

$$
Q=Y'W^Q,\quad K=ZW^K,\quad V=ZW^V
$$

这里 $Y'$ 是 Masked Self-Attention 之后的目标端表示，$Z$ 是 Encoder 输出的 memory。

<Mermaid>
flowchart LR
  Dec["Decoder 当前表示 $$Y'$$"] --> Qc["生成 Query"]
  Mem["Encoder memory $$Z$$"] --> Kc["生成 Key"]
  Mem --> Vc["生成 Value"]
  Qc --> CrossScore["Cross-Attention"]
  Kc --> CrossScore
  Vc --> CrossScore
  CrossScore --> OutC["结合原文后的目标表示"]
</Mermaid>

公式仍然是熟悉的 Attention：

$$
\text{CrossAttention}(Q,K,V)
=
\text{softmax}\left(\frac{QK^\top}{\sqrt{d_k}}\right)V
$$

只是这次含义变了：

- Query 来自 Decoder：我现在要说什么？
- Key 来自 Encoder：原文里哪些位置能回应我？
- Value 来自 Encoder：如果我关注了这些原文位置，可以取走什么语义？

如果继续用演员比喻：Masked Self-Attention 是目标句演员内部先对戏，确认“我前面已经说了什么”；Cross-Attention 则是目标句演员回头看原文演员，确认“我现在该翻译原文里的哪一部分”。

比如在生成“狗”时，Decoder 里当前这个位置可能会强烈关注 Encoder memory 中对应 `dog` 的那一行；生成“你的”时，则可能更关注 `your`。这就是翻译里的对齐感。

### 4) Decoder 里的 Add & Norm 和 FFN

Decoder 每个 Attention 后也会做 Add & Norm。

第一处：

$$
Y'=\text{LayerNorm}(Y+\text{MaskedSelfAttention}(Y))
$$

第二处：

$$
Y''=\text{LayerNorm}(Y'+\text{CrossAttention}(Y',Z))
$$

然后再经过 FFN：

$$
Y_{\text{next}}=\text{LayerNorm}(Y''+\text{FFN}(Y''))
$$

<Mermaid>
flowchart LR
  Y0["输入 Y"] --> MSA["Masked Self-Attention"]
  MSA --> AN1["Add & Norm"]
  AN1 --> CA["Cross-Attention"]
  Z0["Memory Z"] --> CA
  CA --> AN2["Add & Norm"]
  AN2 --> FFND["FFN"]
  FFND --> AN3["Add & Norm"]
  AN3 --> YN["Decoder 层输出"]
</Mermaid>

这部分和 Encoder 的逻辑很像：Attention 负责交换信息，FFN 负责在每个位置内部做非线性加工，Add & Norm 则负责稳定训练并保留原信息。

区别在于 Decoder 有两次 Attention：

1. 第一次看目标端自己，但只能看过去
2. 第二次看 Encoder 的 memory，把原文语义接进来

### 5) Linear + Softmax：从向量变成词

Decoder 最后一层输出后，还只是一个表示矩阵。要真正生成词，还需要把最后一个位置的向量变成词表上的概率分布。

假设最后一个位置的输出向量是 $o_t$，先经过线性层：

$$
\text{logits}=o_tW_{\text{vocab}}+b
$$

其中 $W_{\text{vocab}}$ 会把模型维度投影到词表大小：

$$
\mathbb{R}^{d_{\text{model}}}\rightarrow\mathbb{R}^{|\mathcal{V}|}
$$

然后 softmax：

$$
p_t=\text{softmax}(\text{logits})
$$

<Mermaid>
flowchart LR
  Ot["最后位置向量 $$o_t$$"] --> Linear2["Linear 到词表维度"]
  Linear2 --> Logits["logits"]
  Logits --> Prob["Softmax 概率分布"]
  Prob --> Pick["选择下一个 token"]
</Mermaid>

如果词表里有很多候选词，softmax 会给每个词一个概率。比如：

$$
P(\text{狗})=0.62,\quad
P(\text{猫})=0.11,\quad
P(\text{人})=0.04
$$

模型就会根据解码策略选择下一个 token。最简单的是取概率最高的词，也就是 greedy decoding；更复杂的还有 beam search、top-k、top-p 等方法。这里先不展开，否则会跑出 Transformer 主线。

生成出下一个 token 后，它会被追加到目标序列末尾，再送回 Decoder，继续生成下一个：

<Mermaid>
flowchart LR
  Start["BOS"] --> D1["Decoder"]
  D1 --> W1["我"]
  W1 --> D2["Decoder"]
  D2 --> W2["爱"]
  W2 --> D3["Decoder"]
  D3 --> W3["你的"]
  W3 --> D4["Decoder"]
  D4 --> W4["狗"]
</Mermaid>

所以 Decoder 的本质是循环陈述：

1. 看自己已经说过什么
2. 回看 Encoder 理解出的原文 memory
3. 生成下一个最合适的 token
4. 把这个 token 接回输入，继续下一轮

直到生成 `<EOS>`，也就是 end of sentence，整句翻译才结束。

<Mermaid>
flowchart LR
  History["已生成序列"] --> DecoderLoop["Decoder"]
  MemoryLoop["Encoder memory"] --> DecoderLoop
  DecoderLoop --> NextToken["下一个 token"]
  NextToken --> History
  NextToken --> EOS["若为 EOS 则结束"]
</Mermaid>

到这里，经典 Encoder-Decoder Transformer 的主线就闭合了：Encoder 负责把源句读成 memory，Decoder 负责根据 memory 一步步说出目标句。


## 为什么是Transformer

讲完 Encoder 和 Decoder 后，还剩一个问题：为什么偏偏是 Transformer？

它不是唯一能做翻译的模型。RNN、LSTM、GRU、CNN Seq2Seq 都能做。但 Transformer 真正厉害的地方在于：它把“语义建模”这件事，改造成了特别适合扩大规模、特别适合 GPU 并发、特别适合大数据训练的矩阵计算。

可以粗略概括为三点：

1. 它能比较自然地把模型做大
2. 它能充分利用 GPU 并发
3. 它把长距离依赖的路径压短了，训练效率更高

<Mermaid>
flowchart LR
  T["Transformer"] --> Scale["更容易扩大规模"]
  T --> GPU["更适合 GPU 并发"]
  T --> Path["更短的信息路径"]
  Scale --> Better["更强表达能力"]
  GPU --> Faster["更高训练吞吐"]
  Path --> Learn["更容易学习长距离关系"]
</Mermaid>

### 1) 深度学习为什么喜欢大模型

深度学习里有一个很朴素但很重要的经验：在数据足够、训练足够稳定的前提下，模型规模越大，通常效果越好。

这里的“大”，不只是参数更多，而是整体能力变强：

- 层数更深，可以做更多轮抽象
- 隐藏维度更宽，可以容纳更丰富的语义
- FFN 中间层更大，可以提供更强的非线性加工能力
- Attention head 更多，可以从更多关系角度观察句子

Transformer 正好非常适合做这些扩展。

前面说过，一个 Transformer Block 大致由 Self-Attention、Add & Norm、FFN 组成。其中真正吃参数的大头，往往不是 Attention，而是 FFN。

典型 FFN 是：

$$
\text{FFN}(x)=\sigma(xW_1+b_1)W_2+b_2
$$

如果模型维度是 $d_{\text{model}}$，中间层维度是 $d_{\text{ff}}$，那么 FFN 的主要参数量大约是：

$$
2d_{\text{model}}d_{\text{ff}}
$$

这意味着 Transformer 可以很直接地通过扩大 $d_{\text{model}}$、扩大 $d_{\text{ff}}$、增加层数、增加 head 数来提升容量。

<Mermaid>
flowchart LR
  Small["小模型"] --> Width["扩大 d_model"]
  Small --> FFN["扩大 d_ff"]
  Small --> Depth["堆叠更多层"]
  Small --> Heads["增加 heads"]
  Width --> Big["更大 Transformer"]
  FFN --> Big
  Depth --> Big
  Heads --> Big
</Mermaid>

但“能变大”不等于“变大后还能训练”。Transformer 能撑住规模，靠的是几个结构配合：

- 残差连接让信息有捷径可走，深层时不容易断
- LayerNorm 稳定每层输入输出的分布
- Attention 让每层都能直接交换全局信息
- FFN 提供大规模参数容量，负责局部非线性加工

所以它不是单纯把 MLP 堆大，而是把“全局通信”和“大规模非线性加工”组合到一个稳定的模块里，再一层层堆起来。

从这个角度看，Transformer 的核心不是某个单独公式，而是一个非常适合 scale up 的工程结构。

### 2) Transformer 为什么适合 GPU

GPU 最擅长什么？

不是一个个按顺序做小计算，而是同时做大量相似的矩阵运算。

RNN 类模型的问题在于，它天然有时间顺序依赖。要算第 $t$ 个位置，往往得先算完第 $t-1$ 个位置：

$$
h_t=f(h_{t-1},x_t)
$$

这就像排队传话。第 10 个人想开口，必须等前 9 个人依次说完。

<Mermaid>
flowchart LR
  R1["x1"] --> R2["x2"]
  R2 --> R3["x3"]
  R3 --> R4["x4"]
  R4 --> R5["x5"]
</Mermaid>

而 Transformer 不这样做。

Self-Attention 会把整句表示矩阵一次性拿出来：

$$
X\in\mathbb{R}^{n\times d_{\text{model}}}
$$

然后直接做矩阵乘法：

$$
Q=XW^Q,\quad K=XW^K,\quad V=XW^V
$$

再做：

$$
S=QK^\top
$$

这类操作非常适合 GPU，因为它们都是大块矩阵乘法，也就是 GEMM。

<Mermaid>
flowchart LR
  X["整句矩阵 X"] --> Q["矩阵乘法生成 Q"]
  X --> K["矩阵乘法生成 K"]
  X --> V["矩阵乘法生成 V"]
  Q --> S["QK^T"]
  K --> S
  S --> A["Softmax"]
  A --> O["AV"]
  V --> O
</Mermaid>

这就是 Transformer 的硬件友好性：它让很多 token 的计算同时发生，而不是按 token 顺序一个个推进。

所以在训练时，Transformer 可以同时利用多个维度的并行：

- batch 维度：多条句子一起算
- sequence 维度：一句话里的多个 token 一起算
- hidden 维度：每个向量里的多个维度一起算
- head 维度：多个 attention head 一起算

<Mermaid>
flowchart TD
  GPU["GPU 并发"] --> Batch["batch 并行"]
  GPU --> Seq["sequence 并行"]
  GPU --> Hidden["hidden 维度并行"]
  GPU --> Head["multi-head 并行"]
</Mermaid>

这就是为什么 Transformer 和现代 GPU 的关系非常紧密。它不是只是“算法好”，而是它的计算形态刚好对上了硬件擅长的方向。

### 3) 计算量优势到底在哪里

严格说，Transformer 的 Self-Attention 不是在所有情况下都更省计算。

Self-Attention 的核心打分是：

$$
QK^\top
$$

如果句长是 $n$，维度是 $d$，那么它大致需要：

$$
O(n^2d)
$$

因为每个 token 都要看所有 token，所以会有 $n\times n$ 的注意力矩阵。句子特别长时，这个二次复杂度并不便宜。

但 Transformer 的优势不只看理论 FLOPs，还要看三件事。

第一，Transformer 的计算可以并行。

RNN 可能每一步计算量不大，但它很多步骤必须顺序做；Transformer 单层计算量可能更大，但能一次性并行铺开。在 GPU 上，并行矩阵乘法往往比串行小计算更有效率。

第二，Transformer 的信息路径更短。

在 RNN 中，句首 token 要影响句尾 token，需要沿着时间步一步步传过去：

$$
x_1\rightarrow h_1\rightarrow h_2\rightarrow \dots \rightarrow h_n
$$

路径长度大约是 $O(n)$。

而在 Self-Attention 中，任意两个 token 可以在一层里直接交互：

$$
x_i\rightarrow x_j
$$

路径长度接近 $O(1)$。

<Mermaid>
flowchart LR
  A["RNN: 逐步传递"] --> B["长路径"]
  C["Attention: 直接连接"] --> D["短路径"]
  B --> Hard["长距离依赖更难学"]
  D --> Easy["长距离依赖更容易学"]
</Mermaid>

这对翻译很关键。因为一个词的含义经常由很远处的词决定，比如主语、宾语、修饰关系、指代关系。路径越短，信息越不容易在传递过程中变弱。

第三，Transformer 的主要计算是规则矩阵乘法。

规则矩阵乘法有大量成熟优化：GPU kernel、张量核心、混合精度、批量化、算子融合等。理论复杂度只是一部分，实际训练速度还取决于硬件利用率。

所以 Transformer 的“计算量优势”更准确地说，是：

- 它不一定在 FLOPs 上永远最少
- 但它把计算组织成了更容易并行、更容易优化、更容易堆规模的形式
- 在大数据和大 GPU 集群上，它能把更多计算真正转化成模型能力

### 小结

Transformer 之所以成为主流，不只是因为 Attention 这个想法漂亮，而是因为它同时满足了三个条件：

1. 表达能力强：Self-Attention 负责全局通信，FFN 负责大规模非线性加工
2. 可扩展：宽度、深度、head 数、FFN 维度都能继续放大
3. 硬件友好：核心计算是大规模矩阵乘法，能吃满 GPU 并发

换句话说，Transformer 不只是一个翻译模型结构，它更像是一种把“语言理解”改写成“可并行矩阵计算”的方法。

这也是为什么后来的大语言模型基本都沿着 Transformer 继续放大：模型越大，数据越多，算力越强，这个结构越能显出优势。
