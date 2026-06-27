> This article is the first in a series unpacking the Transformer architecture.
> It is a personal visualization breakdown of the classic Encoder-Decoder architecture.

## Transformer as a Translation Machine

The Transformer architecture made its debut in Google's paper *Attention Is All You Need*, marking a milestone in the field of machine learning. It was originally created to solve the Seq2seq (Sequence to Sequence) machine translation problem more efficiently.

So, I treat the Transformer as a translation machine.

Here is the mini architecture diagram:
<!-- Mermaid - Mini Architecture -->

<Mermaid>
flowchart LR
  Source["Source Sentence"] --> Encoders["Encoders"]
  Encoders --> Memory["Semantic Context"]
  Memory --> Decoders["Decoders"]
  Decoders --> Target["Target Sentence"]
</Mermaid>

In the diagram, the original sentence is first processed by the Encoder into an intermediate semantic context, and then the Decoder generates the resulting sentence token by token (**autoregressively**) based on that context.

The whole process is an act of translation. To understand the classic Transformer, we first need to understand the act of translation itself.

---

## The Difficulty of Translation

In the physical world, translation can be broken into two steps:

1. The translator receives the original message, **understands** the context using their own knowledge, and temporarily stores the digested context.
2. The translator expresses the message based on that context.

The Transformer does the same. The Encoder corresponds to the first step; the Decoder corresponds to the second.

However, the machine process is not as complex as human logic. Since we are already used to translation, let's boldly sort out the machine process.

It seems like only two steps, but the difficulty lies in: how do we implement them?

## Encoder: How Understanding is Implemented

This is how the Encoder works. For example, if the source sentence is $I love your dog$:

### Embedding

Embedding consists of several steps:

1. Tokenization
2. Embedding
3. Position Embedding

<Mermaid>
flowchart LR
  A["Original: I love your dog"] --> B["Tokenization: X=(x_1,x_2,...,x_n)"]
  B --> C["Embedding: $$x_i \in \mathbb{R}^{d_{\text{model}}}$$"]
  C --> D["Position Embedding: $$x_i + p_i$$"]
  D --> E["Input Matrix: $$X \in \mathbb{R}^{n \times d_{\text{model}}}$$"]
</Mermaid>

The original Sequence is broken into multiple fragments through Tokenization, becoming a Token sequence (different algorithms split differently; to keep the example simple, all tokens below are split as whole words). Let the number of fragments be $n$, represented as:

$$
X=(x_1,x_2,\dots,x_n)
$$

In the process above, $I love your dog$ is first split into $(I, love, your, dog)$.

Then, each token is mathematically modeled. For example, the first token can be written as:

$$
x_1=[0.12,-0.31,0.77,\dots]\in \mathbb{R}^{d_{\text{model}}}
$$

Stacking the whole sentence gives the input matrix:

$$
X\in\mathbb{R}^{n\times d_{\text{model}}}
$$

Assume our model dimension is 6. Each token is then converted into a 6-dimensional vector representation:

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

From a geometric perspective, these are 4 points floating in a 6-dimensional space. They are currently "isolated": only word meanings exist, no sentence meaning. $I$ is just I, $love$ is just love, and $dog$ is still just the dictionary definition of dog.

How do we make these 4 words share information with each other, causing each to be influenced and corrected (as vector updates)? This is what Attention will do next.

(Note: the embedding step above only completes the word-meaning vectorization. A third step also needs to carry each token's position information within the source sentence. This is done by Positional Encoding; with positional information, the sentence-level context for Self-Attention can be effective. We won't expand on it here.)

### 1) Self-Attention: The Self-Attention Mechanism

Attention solves the following problem: each row of the representation matrix should complete its own semantic enrichment. The flow is as follows:

<Mermaid>
flowchart LR
  X["Input Matrix $$X$$"] --> QKV["Generate $$Q,K,V$$"]
  QKV --> Score["Score Matrix $$S=QK^\top$$"]
  Score --> Scale["Scale $$S/\sqrt{d_k}$$"]
  Scale --> Softmax["Attention Matrix $$A=\operatorname{softmax}(S/\sqrt{d_k})$$"]
  Softmax --> Mix["Output Matrix $$O=AV$$"]
  Mix --> Out["Semantically Enriched Matrix"]
</Mermaid>

Mathematically:

$$
\text{Attention}(Q,K,V)=\text{softmax}\left(\frac{QK^\top}{\sqrt{d_k}}\right)V
$$

Although the formula has many variables, from a functional perspective it is very clear.

Let's unpack it step by step.

> The Attention process involves a lot of matrix operations. If you are not familiar with them, I recommend reading [The Geometric Intuition of Matrices](/blog/matrix_geometric_intuition) first.

#### Q, K, V

Our input is $X$, and inside the Attention function we see $Q,K,V$. So what is the relationship between $X$ and $Q,K,V$?

In fact, $Q,K,V$ are the result of projecting the source sentence matrix $X$ from three different angles using the **learned weights** — these are the `weight matrices`.

The weight matrices are usually denoted as $W$, like a repeatedly refined `"stage script"`:
> The sentence $X$ is a play; each token is an actor. At first, each actor only knows their own role. Whether the whole play is perfect (i.e., whether the semantics are perfect) depends on whether the actors can fully **rehearse with** (Attention) each other.

In Self-Attention, this script $W$ is split into three parts: $W^Q, W^K, W^V$. They are not part of the input sentence; they are model parameters. Every time the input representation matrix $X$ enters a layer, these three groups of parameters generate the corresponding $Q, K, V$ matrices.

> At the beginning of training, the entries of $W$ are just random numbers. This initial matrix is unlikely to produce good Seq2seq output, but through training the random weights are gradually optimized into more effective matrices.

Implementation-wise, the current representation matrix $X$ is first projected into three matrices:

$$
Q=XW^Q,\quad K=XW^K,\quad V=XW^V
$$

If we only look at the $i$-th row of each matrix, they represent:

- $q_i$: "What am I looking for?" (which other actors should I rehearse with)
- $k_i$: "How am I matched?" (what role am I playing)
- $v_i$: "What content can I contribute?" (what is my part in the scene)

$X$ is multiplied by $W$ all at once. This is equivalent to each row generating its own $q_i, k_i, v_i$:

$$
q_i=x_iW^Q,\quad k_i=x_iW^K,\quad v_i=x_iW^V
$$

> Note: Although the weight matrix $W$ is split into $W^Q, W^K, W^V$, in engineering it is usually represented as one large matrix, and the Q, K, V matrices are obtained by slicing during computation. This is done to leverage the efficiency of large matrix multiplication on GPUs.

#### Q, K, V Generation Example

Assume the current input matrix $X$ is $4\times 6$, and we have the corresponding $W^Q, W^K, W^V$. Note: these numbers are only for demonstrating the matrix pipeline; they are not real trained parameters.

> The training of the weight matrices is a topic large enough for its own article; we won't expand on it here.

The three weight matrices can be written as:

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

Multiplying the whole sentence at once gives three result matrices:

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

The same $X$ is projected into three different weight matrices; these three matrices form the logical core of Attention.

> Note 1: $Q, K, V$ are matrices representing the whole sentence, not temporary $1\times n$ vectors. Later, when we extract $q_{love}$ or $k_I$, it is simply because they are particular rows of the $Q$ and $K$ matrices.

> Note 2: Here we let $W^V$ output 6 dimensions so that the final result can return directly to the main model dimension $d_{model}$. In real engineering, it is common to output a shorter $d_v$ first, and then use an additional $W_O$ to project back to $d_{\text{model}}$.

#### $Q \cdot K^\top$ (Dot Product)

After obtaining the three matrices, the first step of the Attention (rehearsing) process is the $QK$ matrix operation. Specifically, it is $Q \cdot K^\top$, transposing $K$, because the dimensions $6\times 3$ and $3\times 6$ are valid for matrix multiplication. We temporarily call the result $S$:

$$
S=QK^\top\in\mathbb{R}^{n\times n}
$$

<Mermaid>
flowchart LR
  Qm["$$Q\in\mathbb{R}^{n\times d_k}$$"] --> M["Matrix Multiplication"]
  Kt["$$K^\top\in\mathbb{R}^{d_k\times n}$$"] --> M
  M --> S["$$S=QK^\top\in\mathbb{R}^{n\times n}$$"]
  S --> Cell["S_ij: score of row i attending to row j"]
</Mermaid>

Here, the $i$-th row means:
- "Which token is the $i$-th token row looking at?"
- The $j$-th column means "the score it assigns to the $j$-th token row."

Let's do a full expansion for $love$ looking at $I$. To avoid skipping, we first put the complete $Q$ and $K$ again:

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

Thus:

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

Their relevance score (how much they need to rehearse together) is:

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

This is a dot product of two vectors, resulting in a single number. This number reflects how relevant word $i$ is to the word $love$; it indicates how much the word $love$ should draw supplementary information from $i$ when it is finally translated. In our stage-play metaphor, it represents how much the actor $love$ should rehearse with actor $i$ in order to perform the script perfectly:

Since our model dimension is very small, this dot product can be visualized as the radar chart below, where overlap reflects relevance.

<Mermaid>
radar-beta
  title "3D Radar Overlap of q_love and k_I (Score≈0.564)"
  axis d1["Dimension 1"], d2["Dimension 2"], d3["Dimension 3"]
  curve q["q_love"]{0.550, 0.780, 1.550}
  curve k["k_I"]{0.422, 0.227, 0.100}
  max 1.6
  min 0
</Mermaid>

Don't forget, we only took one pair of tokens as an example; the actual computation is performed on the whole matrix in one go. This is the first step of Attention. The product of this step can be said to be: **every actor now knows how much they need to rehearse with every other actor in the script.**

#### Scaled - $\sqrt{d_k}$ and Softmax

After obtaining the scores, each row is first divided by $\sqrt{d_k}$ for scaling, and then passed through softmax:

To explain these two steps clearly, we first need to understand how softmax works:

After softmax, each value $s_{ij}$ in the similarity matrix changes from a score to a proportion (score rate). For token $i$, it means the proportion of its total attention that is allocated to token $j$.

The classic mathematical form is:

$$
\operatorname{softmax}(x_i)=
\frac{e^{x_i}}{\sum_{j=1}^{n} e^{x_j}},
\quad i=1,2,\dots,n
$$

At this point, for any row held by token $i$, every value becomes a proportion; higher scores get larger proportions, and the sum is 1. This process can be seen as **attention allocation**.

In the softmax formula, it depends on $e$, which means that if there are extreme values, all the weight will focus on the largest region.

This is where $\sqrt{d_k}$ comes in. It is called the scaling factor. Its purpose is to control the range of $Q \cdot K^T$, keeping the variance of the resulting matrix within a certain interval to avoid extreme polarization.

> Why is the scaling factor $\sqrt{d_k}$ and not something else? In the original Transformer paper, this value comes from an assumption: the vectors $q$ and $k$ inside $Q$ and $K$ have mean 0 and variance 1. Their dot product then has variance $d_k$. Softmax wants data with variance around 1, so the scaling factor that makes the scaled result have variance 1 is $\sqrt{d_k}$. This assumption is a common convention in the machine learning field, not the only solution. If interested, see these two articles: [苏剑林 - On Transformer Initialization, Parameterization, and Standardization](https://kexue.fm/archives/8620#NTK%E5%8F%82%E6%95%B0%E5%8C%96) and [苏剑林 - Entropy Invariance of Attention Scale](https://kexue.fm/archives/8823#%E7%86%B5%E4%B8%8D%E5%8F%98%E6%80%A7).

#### V

Through the previous step, we obtain a proportion matrix. It means every token knows the weight of semantic supplementation it should get from other tokens when the final source sentence meaning is generated.

In the stage-play metaphor, this means every actor knows, under the script $W$, with which other colleagues and to what degree they should rehearse in order to perform perfectly.

But knowing how much to rehearse is not enough; we also need to know the other actor's part. This is where the weight matrix $V$ comes in.

The overall process is:

<Mermaid>
flowchart LR
  Scores["Row i scores: s_i"] --> Softmax2["softmax"]
  Softmax2 --> A["Attention weights $$a_i$$"]
  Vmat["Value Matrix $$V$$"] --> Mix2["Weighted sum"]
  A --> Mix2
  Mix2 --> O["Output row $$o_i$$"]
</Mermaid>

This is the $i$-th row of the output matrix $O$, i.e., the new representation of the $i$-th token after absorbing information from the whole scene.

For example, in a certain layer, the attention weights of $love$ over the whole sentence might be:

$$
a_{love}=[0.30,0.45,0.10,0.15]
$$

Then its new representation is:

$$
o_{love}=0.30v_I+0.45v_{love}+0.10v_{your}+0.15v_{dog}
$$

This step is the "vector correction". The coordinate point originally occupied by $love$ absorbs information from tokens such as $I, your, dog$. It is no longer an isolated verb from the dictionary; it becomes a "love emitted by I and directed toward some object." Here, $I$ participates in the semantic correction of $love$. This process is called "Attend To" in terminology. It also matches our stage-play metaphor: the actor $I$ "Attends To" the scene of the actor $love$.

#### Summary

Reviewing the whole process, Self-Attention can be divided into four steps:

1. Project the representation matrix into three perspectives using $W^Q, W^K, W^V$.
2. Use $QK^\top$ to get the rehearsal strength between token rows.
3. Use softmax to turn strength into attention weights.
4. Use these weights to mix $V$, completing the vector correction for each token.

In other words, each layer of Attention moves the word vectors once in space. As layers stack up, each token's vector gradually carries the shadow of the whole sentence.

Readers can once again imagine our stage-play metaphor here.

#### Multi-Head Attention

Multi-Head Attention is not complicated. It is the same group of actors switching rehearsal methods, from one angle to multiple angles (Heads):
- Some heads focus on grammatical dependencies.
- Some heads focus on anaphoric (referential) relations.
- Some heads focus on semantic collocations.

If the input is still denoted as $X\in\mathbb{R}^{n\times d_{\text{model}}}$, the $i$-th head has its own set of projection matrices:

$$
\begin{align}
Q_i&=XW_i^Q,\quad W_i^Q\in\mathbb{R}^{d_{\text{model}}\times d_k}\\
K_i&=XW_i^K,\quad W_i^K\in\mathbb{R}^{d_{\text{model}}\times d_k}\\
V_i&=XW_i^V,\quad W_i^V\in\mathbb{R}^{d_{\text{model}}\times d_v}
\end{align}
$$

So the output of the $i$-th head is:

$$
\begin{align}
\text{head}_i
&=\text{Attention}(Q_i,K_i,V_i)\\
&=\text{Attention}(XW_i^Q,XW_i^K,XW_i^V)
\end{align}
$$

The results of multiple heads are concatenated and then projected back to the main model dimension by $W^O$ to be passed to subsequent layers:

$$
\begin{align}
\text{MultiHead}(Q,K,V)
&=\operatorname{Concat}(\text{head}_1,\text{head}_2,\dots,\text{head}_h)W^O\\
W^O&\in\mathbb{R}^{hd_v\times d_{\text{model}}}
\end{align}
$$

Here, $d_k=d_v=d_{\text{model}}/h$; each head only looks at a small subspace; after $h$ heads are concatenated, the dimension returns to $d_{\text{model}}$. Readers can derive the relationship between the $Q,K,V$ matrix sizes and the model dimension $d_{model}$ themselves.

Using our example above, the process diagram is:

<Mermaid>
flowchart LR
  Xm["Input $$X$$"] --> Head1["Head 1: Grammatical Dependencies"]
  Xm --> Head2["Head 2: Referential Relations"]
  Xm --> Head3["Head 3: Semantic Collocations"]
  Head1 --> Cat["Concat Multi-Head Results"]
  Head2 --> Cat
  Head3 --> Cat
  Cat --> WO["Multiply by $$W_O$$ to return to main space"]
  WO --> Out2["Multi-Head Output"]
</Mermaid>

> Note: Multi-head means there are multiple attention weight matrices, and each head has its own independent $W_i$ parameters.

### 2) Add & Norm

Self-Attention has already corrected each token's vector once. But if we directly pass this corrected result down, deep training can easily become unstable: the values may drift larger and larger, and the original word meaning may gradually be lost.

So every layer of the Transformer structure adds a stability component:

$$
X'=\text{LayerNorm}(X+\text{SelfAttention}(X))
$$

This has two steps:

- **Add**: Add the output of Self-Attention to the original input $X$ to preserve the original signal.
- **LayerNorm**: Normalize the summed result to push the values back into a stable range.

<Mermaid>
flowchart LR
  X0["Input X"] --> Plus["Residual Add +"]
  SA["Self-Attention(X)"] --> Plus
  Plus --> LN["LayerNorm"]
  LN --> Xp["X'"]
</Mermaid>

After FFN, it is done again:

$$
X_{next}=\text{LayerNorm}(X'+\text{FFN}(X'))
$$

In other words, **there is an Add & Norm both before and after Attention and FFN**. Together they ensure the stable flow of information.

#### Add — Residual Connection

Assume after Self-Attention, the vector of token `love` changes from:

$$
x_{love}=[0.4,0.5,0.6]
$$

to:

$$
\text{SelfAttention}(x_{love})=[2.1,-1.0,0.3]
$$

Without the residual connection, the next layer would receive `[2.1, -1.0, 0.3]`. This vector is already quite different from the original `love` vector `[0.4, 0.5, 0.6]`. If each layer does this, after 6 layers (the number of Encoder Layers in the original paper) the model may completely forget what `love` originally meant.

With the residual connection:

$$
\begin{align}
x_{love}'
&= x_{love} + \text{SelfAttention}(x_{love}) \\
&= [0.4,0.5,0.6] + [2.1,-1.0,0.3] \\
&= [2.5,-0.5,0.9]
\end{align}
$$

Although the numbers have changed, the vector `[2.5, -0.5, 0.9]` still **contains the original `[0.4, 0.5, 0.6]` component**. The original signal is not discarded; it is preserved as a "base."

In the stage-play metaphor: **Attention lets every actor reinterpret their role, but Add lets the actor not forget "who they originally were."**

In addition to preserving information, Add has another important role:

Deep learning adjusts model parameters through the **backpropagation mechanism**, which depends directly on gradients. Gradient computation is a chain multiplication from deep layers (near the output) to shallow layers (near the input). Without any treatment, the gradient can become close to zero early on (**vanishing gradient**), so the parameters of shallow layers cannot be trained effectively. With $Add$, the gradient can stably reach the shallow layers during computation.

The core lies in the mathematical form of the residual connection: the derivative of **$y = x + f(x)$** naturally contains a +1 (identity matrix). Let's expand this:

1. Without Add

Suppose a layer is:

$$
y = f(x)
$$

where $f$ may contain Attention, FFN, LayerNorm, etc.

Backpropagation computes the gradient:

$$
\frac{\partial L}{\partial x} = \frac{\partial L}{\partial y} \cdot \frac{\partial y}{\partial x}
= \frac{\partial L}{\partial y} \cdot f'(x)
$$

If the network is very deep (still taking 6 layers as an example):

$$
x_6 = f_6(f_5(f_4(f_3(f_2(f_1(x))))))
$$

Then the gradient from Loss back to $x_1$ requires successive multiplication:

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

If the norm of each $f_i'(x)$ is less than 1 (for example, 0.6), then after 6 layers:

$$
0.6^6 \approx 0.047
$$

The closer to the shallow layers, the smaller the gradient, and the slower the parameter updates. This is the vanishing gradient problem.

2. With Add

The residual connection becomes:

$$
y = x + f(x)
$$

The derivative becomes:

$$
\frac{\partial y}{\partial x} = I + f'(x)
$$

where $I$ is the identity matrix.

Backpropagation:

$$
\frac{\partial L}{\partial x}
= \frac{\partial L}{\partial y} \cdot \frac{\partial y}{\partial x}
= \frac{\partial L}{\partial y} \cdot (I + f'(x))
$$

Even if $f'(x)$ is small, the gradient still retains a full $\frac{\partial L}{\partial y}$ term.

#### Layer Normalization

Without Norm, the residual-added values would accumulate layer by layer. For example, assume each layer's correction is similar:

$$
[0.4,0.5,0.6] + [2.1,-1.0,0.3] = [2.5,-0.5,0.9]
$$

Continuing to add in the second layer:

$$
[2.5,-0.5,0.9] + [2.1,-1.0,0.3] = [4.6,-1.5,1.2]
$$

By the sixth layer, the vector might become:

$$
[13.0,-5.5,2.4]
$$

The values become larger and larger, and the distribution becomes more and more unstable. Later layers receiving this input will find training very difficult.

This is where LayerNorm comes in: for each token vector, normalize it so that the **mean is pushed to 0 and the variance to 1**, then fine-tune with learnable $\gamma$ and $\beta$:

$$
\text{LayerNorm}(x)=\gamma\cdot\frac{x-\mu}{\sqrt{\sigma^2+\varepsilon}}+\beta
$$

> In the original Transformer paper, Add and Norm use the Post-Norm form: add first, then normalize. Many modern large models (such as GPT and LLaMA) use Pre-Norm:
> $$
> X_{next}=X+\text{FFN}(\text{LayerNorm}(X))
> $$

### 3) FFN: MLP is Still What You Need

The title of the paper *Attention Is All You Need* only emphasizes Attention, but the real core of Transformer is **Attention + FFN**.

- **Attention** solves "how tokens exchange information with each other."
- **FFN** solves "after each token receives information, how does it reprocess it internally."

If each token is an actor, then Attention is the actors rehearsing with each other, while FFN is the actor digesting their own script after rehearsal to perform better.

The FFN formula is:

$$
\text{FFN}(x)=\max(0,xW_1+b_1)W_2+b_2
$$

where $\max(0, \dots)$ is the ReLU activation; modern models also commonly use GELU.

<Mermaid>
flowchart LR
  Xi["Single position x_i'"] --> Up["W1 up-project"]
  Up --> Act["ReLU / GELU non-linearity"]
  Act --> Down["W2 down-project to d_model"]
  Down --> Residual["Residual add with x_i'"]
  Residual --> LN2["LayerNorm"]
  LN2 --> Next["Next layer input"]
</Mermaid>

#### Attention + FFN?

The output of Attention is essentially:

$$
\text{Attention Output} = \sum_j a_{ij} v_j
$$

That is, the new vector at each position is a **weighted average** of the Value vectors of other positions. It is still a linear combination in the input space, only with different weights.

If there were only Attention, no matter how many layers are stacked, what the model could learn would be limited: **it can only redistribute existing information, not create new, more abstract features**.

FFN's role is to give the model **non-linear transformation capability**, allowing it to learn patterns such as "if a feature exists, enhance it; if not, suppress it."

> FFN is shared within a single layer:
> For the $i$-th token:
> $$
> x_i' \rightarrow \text{FFN}(x_i')
> $$
> For the $j$-th token:
> $$
> x_j' \rightarrow \text{FFN}(x_j')
> $$
> Both use the same set of parameters $W_1, b_1, W_2, b_2$.

#### Up-Project, Activate, Down-Project

The FFN process is `up-project → activate → down-project`:

$$
d_{\text{model}} \rightarrow d_{\text{ff}} \rightarrow d_{\text{model}}
$$

In the original paper, $d_{\text{ff}} = 4 \times d_{\text{model}}$. For example, if $d_{\text{model}} = 512$, the FFN intermediate dimension is 2048.

The benefits are:

1. **Up-project**: Expand semantics into a higher-dimensional space, making originally entangled features easier to separate.
2. **Activate**: Use a non-linear function to turn linearly non-separable problems into separable ones.
3. **Down-project**: Compress the result back to $d_{\text{model}}$, so the next Encoder layer can continue.

#### Key-Value Memory

In addition to non-linear transformation, FFN also plays an important role: **storing knowledge**.

In recent years, many interpretability studies have suggested that FFN can be understood as a kind of **Key-Value memory**.

- $W_1$ is responsible for matching the input vector to certain "keys" (specific neurons).
- $W_2$ is responsible for outputting the corresponding "values" (semantic supplements).

Each hidden-layer neuron is like a knowledge entry. When the input vector matches an entry, that neuron is activated, and then through $W_2$ it writes the relevant knowledge into the residual stream.

For example, when `Michael Jordan` appears in the input:

1. Its vector is projected by $W_1$.
2. Certain neurons are activated; these neurons have learned the "Michael Jordan pattern" during training.
3. The activated neurons output a direction through $W_2$, corresponding to semantics such as "basketball player," "NBA," and "Chicago Bulls."
4. This output is added to the original vector through the residual connection.

So, the fact that Michael Jordan is a basketball player **is not in the input token, but in the FFN weights**.

Because FFN layers carry so much information, the main parameters of a Transformer model are not in Attention, but in FFN:

$$
\text{FFN parameters per layer} \approx 2 \times d_{\text{model}} \times d_{\text{ff}}
$$

If $d_{\text{ff}} = 4 \times d_{\text{model}}$, then the FFN parameters are several times the Attention parameters.

So "making the model bigger" largely means expanding the hidden dimension of FFN. The larger the FFN, the more knowledge entries it can store and process, and the stronger the model usually is.

> Just like after Attention, the output of FFN also goes through an Add & Norm:
> $$
> X_{next}=\text{LayerNorm}(X'+\text{FFN}(X'))
> $$

At this point, the flow within one layer is complete. The output is passed to the next layer. If this is already the last layer of the Encoder stack, the Encoder outputs the final context representation:

$$
Z=(z_1,z_2,\dots,z_n)
$$

Let's call $Z$ the memory (the paper *Attention Is All You Need* does not name it, but most code implementations call it memory). It can be understood as the model's "semantic memory after understanding," and is then handed over to the Decoder to complete expression and generation.

## Decoder: Expression

After the Encoder reads the source sentence into memory $Z$, the Decoder is responsible for generating the target sentence word by word based on this memory.

It does not output the whole sentence at once, but generates it **autoregressively** one token at a time:

$$
\text{<BOS>} \rightarrow \text{我} \rightarrow \text{爱} \rightarrow \text{你的} \rightarrow \text{狗}
$$

> `<BOS>` is begin of sentence.  
> `<EOS>` is end of sentence.  
> When `<EOS>` is generated, the whole sentence translation is complete.

The input to the Decoder is not the source sentence $X$, but the part already generated on the target side. After Embedding and Position Embedding, it becomes matrix $Y$. Its processing flow is as follows:

<Mermaid>
flowchart LR
  Tgt["Generated Target Sequence"] --> Emb["Target-Side Embedding"]
  Emb --> Masked["Masked Self-Attention"]
  Masked --> Cross["Cross-Attention"]
  Memory["Encoder memory Z"] --> Cross
  Cross --> FFN["FFN"]
  FFN --> Linear["Linear + Softmax"]
  Linear --> Next["Next token"]
</Mermaid>

In the Decoder, there are notably two Attention layers.

### Masked Self-Attention

The first Attention layer in the Decoder is very similar to the Encoder's, but with an additional mask: **when generating the $t$-th word, it cannot see positions $t+1$ and beyond. Otherwise, during training it would directly copy the answer.**

> The Decoder has two Attention layers, each with its own different weight matrix $W$.

Formula-wise, a mask matrix $M$ is simply added before softmax:

$$
\text{MaskedAttention}(Q_Y,K_Y,V_Y)=
\text{softmax}\left(\frac{Q_YK_Y^\top+M}{\sqrt{d_k}}\right)V_Y
$$

Allowed positions are filled with $0$, and future positions are filled with $-\infty$. After softmax, the probability of future positions becomes 0.

> When the target side has 4 positions, the mask looks like this:
> $$
> M=\begin{bmatrix}
> 0 & -\infty & -\infty & -\infty\\
> 0 & 0 & -\infty & -\infty\\
> 0 & 0 & 0 & -\infty\\
> 0 & 0 & 0 & 0
> \end{bmatrix}
> $$
> By inserting extremely small values, subsequent positions are blocked.

### Cross-Attention

The Cross in Cross-Attention comes from the fact that it crosses two sequences: the Decoder's current sequence and the Encoder's source sequence.

Its key difference from Self-Attention is that $Q$ comes through the Masked layer, while $K,V$ come from the Encoder memory:

$$
Q=Y'W^Q,\quad K=ZW^K,\quad V=ZW^V
$$

> Here $Y'$ is the output of Masked Self-Attention, and $Z$ is the final memory output by the Encoder.

The meaning is: the Decoder's current position uses the Query to ask, "Which positions in the original sentence are relevant to what I am translating now?" and then takes away the semantics from the corresponding Value.

For example, when generating "狗", the Decoder's Cross-Attention will strongly focus on the row corresponding to `dog` in the Encoder memory; when generating "你的", it will focus more on `your`. This is the "alignment" in translation.

### Example: Two Moments

We use the source sentence `I love your dog` and pick two autoregressive moments to demonstrate:

$$
\text{<BOS>} \rightarrow \text{我} \rightarrow \text{爱} \rightarrow \text{你的} \rightarrow \text{狗}
$$

**Moment 1: Generating "我"**

The Decoder input is only `<BOS>`. After Masked Self-Attention, it knows "the sentence has just begun." The Cross-Attention Query takes this state and asks the original memory:

> "What is the beginning of the source sentence? What should be translated first?"

The row corresponding to `I` in the original memory responds most strongly, so the Decoder outputs "我".

**Moment 2: Generating "狗"**

By now the Decoder has already generated `<BOS> 我 爱 你的`. The Masked Self-Attention sees this prefix and forms the state $Y'$: next, a noun is needed. The Cross-Attention Query takes this state and asks the original sentence:

> "I have already said '我 爱 你的'. Next I need a noun. Which word in the original sentence should be translated?"

The row corresponding to `dog` in the original memory responds most strongly, so the Decoder outputs "狗".

Each generated token is appended back to the target sequence, and the next round continues.

### Add & Norm

Within a Decoder layer, there are two Attention layers, each followed by Add & Norm, and finally FFN:

> $$
> Y'=\text{LayerNorm}(Y+\text{MaskedSelfAttention}(Y))
> $$
> $$
> Y''=\text{LayerNorm}(Y'+\text{CrossAttention}(Y',Z))
> $$
> $$
> Y_{\text{next}}=\text{LayerNorm}(Y''+\text{FFN}(Y''))
> $$

We won't expand on this here.

After this step, the current Decoder layer's task is complete. The next step is the same as in the Encoder: if the current layer is not the last layer, it continues to be passed to the next Encoder layer; otherwise, the output of the last layer goes to the next step.

### Linear + Softmax

The Decoder's last layer outputs a matrix. In autoregressive generation, we only need the **last position** vector $o_t$ to decide the next word. But $o_t$ is still in the internal semantic space $\mathbb{R}^{d_{\text{model}}}$; to turn it into a probability over the vocabulary, two steps are needed.

**Step 1: Linear Projection**

Through a linear layer, $o_t$ is mapped to the vocabulary dimension:

$$
\text{logits}=o_tW_{\text{vocab}}+b
$$

$$
\mathbb{R}^{d_{\text{model}}}\rightarrow\mathbb{R}^{|\mathcal{V}|}
$$

Each row of $W_{\text{vocab}}$ corresponds to a word in the vocabulary. After projection, each dimension represents a "score" for a word; the higher the score, the more suitable that word is as the next token.

> For example, if the vocabulary has 50,000 words, logits is a 50,000-dimensional vector. Suppose the score for "狗" is 4.2, "猫" is 1.5, and "人" is 0.3; the model is currently most inclined to generate "狗".

**Step 2: Softmax**

Softmax turns logits into a valid probability distribution, with all word probabilities summing to 1:

$$
p_t=\text{softmax}(\text{logits})
$$

> Continuing the example:
> $$
> P(\text{狗})=0.62,\quad P(\text{猫})=0.11,\quad P(\text{人})=0.04
> $$

<Mermaid>
flowchart LR
  Ot["Last position vector $$o_t$$"] --> Linear["Linear projection to vocabulary"]
  Linear --> Logits["logits"]
  Logits --> Softmax["Softmax"]
  Softmax --> Prob["Vocabulary probability distribution"]
  Prob --> Pick["Select next token"]
</Mermaid>

> Finally, the model selects the next token based on this probability distribution. There are multiple selection strategies; the paper uses `beam search`, which we won't expand on here.
> The selected token is appended back to the target sequence and sent into the Decoder for the next round. This continues until `<EOS>` is generated, and the whole sentence translation ends.
> At this point, the main line of the classic Encoder-Decoder Transformer is closed: the Encoder is responsible for reading the source sentence into memory, and the Decoder is responsible for saying the target sentence step by step based on that memory.

## Conclusion

To this point, we have walked through the entire classic Transformer architecture. This typical structure was proposed to solve the machine translation `seq2seq` problem. From the start, we likened it to a translation machine, then dissected it layer by layer from encoder-decoder to the inner layers, from the initial tokenization to the full-sentence semantic construction of memory, and then to the Decoder's word-by-word output of the translation, completely explaining the working mechanism and purpose of each layer and component.

After mastering the structure of the translation machine, we can move toward the currently hot field of large model knowledge.
