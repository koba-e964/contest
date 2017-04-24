#include <iostream>
#include <algorithm>
#include <vector>
#include <cmath>
#include <map>
using namespace std;
  
#define rep(i, n) for (int i = 0; i < int(n); ++i)
typedef long long i64;
/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid M. Note that constructing this tree requires the identity
 * element of M and the operation of M.
 * Header requirement: vector, algorithm
 * Verified by AtCoder ABC017-D (http://abc017.contest.atcoder.jp/submissions/660402)
 */
template<class I, class BiOp = I (*) (I, I)>
class SegTree {
  int n;
  std::vector<I> dat;
  BiOp op;
  I e;
public:
  SegTree(int n_, BiOp op, I e) : op(op), e(e) {
    n = 1;
    while (n < n_) { n *= 2; } // n is a power of 2
    dat.resize(2 * n);
    for (int i = 0; i < 2 * n - 1; i++) {
      dat[i] = e;
    }
  }
  /* ary[k] <- v */
  void update(int k, I v) {
    k += n - 1;
    dat[k] = v;
    while (k > 0) {
      k = (k - 1) / 2;
      dat[k] = op(dat[2 * k + 1], dat[2 * k + 2]);
    }
  }
  void update_array(int k, int len, const I *vals) {
    for (int i = 0; i < len; ++i) {
      update(k + i, vals[i]);
    }
  }
  /*
    Updates all elements. O(n)
   */
  void update_all(const I *vals, int len) {
    for (int k = 0; k < std::min(n, len); ++k) {
      dat[k + n - 1] = vals[k];
    }
    for (int k = std::min(n, len); k < n; ++k) {
      dat[k + n - 1] = e;
    }
    for (int b = n / 2; b >= 1; b /= 2) {
      for (int k = 0; k < b; ++k) {
	dat[k + b - 1] = op(dat[k * 2 + b * 2 - 1], dat[k * 2 + b * 2]);
      }
    }
  }
  /* l,r are for simplicity */
  I querySub(int a, int b, int k, int l, int r) const {
    // [a,b) and  [l,r) intersects?
    if (r <= a || b <= l) return e;
    if (a <= l && r <= b) return dat[k];
    I vl = querySub(a, b, 2 * k + 1, l, (l + r) / 2);
    I vr = querySub(a, b, 2 * k + 2, (l + r) / 2, r);
    return op(vl, vr);
  }
  /* [a, b] (note: inclusive) */
  I query(int a, int b) const {
    return querySub(a, b + 1, 0, 0, n);
  }
};

const i64 mod = 1e9 + 7;
i64 powmod(i64 x, i64 e) {
i64 sum = 1;
i64 cur = x;
while(e>0) {
if (e% 2==1){sum = sum * cur % mod;}
cur=cur*cur% mod;
e/=2;
}
return sum;
}

struct pmul{
i64 operator()(i64 x, i64 y)const{
return x*y%(mod-1);
}
};

int main(void) {
int m;cin>>m;
map<int, i64> freq;
rep(i, m) {
int v;cin>>v;freq[v] += 1;
}
i64 prod = 1;
vector<pair<i64, i64> > tpp;
for(map<int,i64>::iterator it=freq.begin(); freq.end()!=it;++it){
i64 p=it->first;
i64 e=it->second;
tpp.push_back(make_pair(p, e));
}
int n = tpp.size();
SegTree<i64, pmul> st(n, pmul(), 1);
rep(i,n) {
st.update(i,tpp[i].second+1);
}
rep(i, n) {
i64 p = tpp[i].first;
i64 e = tpp[i].second;
e=e*(e+1)/2;
e%=mod-1;
e=e*st.query(0,i-1)%(mod-1);
e=e*st.query(i+1,n-1)%(mod-1);
prod=prod*powmod(p,e)%mod;
}
cout<<prod<<endl;
}
