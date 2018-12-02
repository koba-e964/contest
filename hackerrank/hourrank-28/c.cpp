#include <iostream>
#include <fstream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>
const int DEBUG = 0;

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

using namespace std;

// http://tsutaj.hatenablog.com/entry/2017/03/30/224339
struct LazySegmentTree {
private:
    int n;
    vector<int> node, lazy;

public:
  LazySegmentTree(){}
  void init(vector<int> v) {
    int sz = (int)v.size();
    n = 1; while(n < sz) n *= 2;
    node.resize(2*n-1, 0);
    lazy.resize(2*n-1, 1);

    for(int i=0; i<sz; i++) node[i+n-1] = v[i];
    for(int i=n-2; i>=0; i--) node[i] = node[i*2+1] + node[i*2+2];
  }
  void eval(int k, int l, int r) {

    if(lazy[k] != 0) {
      node[k] *= lazy[k];

      if(r - l > 1) {
	lazy[2*k+1] *= lazy[k];
	lazy[2*k+2] *= lazy[k];
      }

      lazy[k] = 1;
    }
  }
  void add(int a, int b, ll x, int k=0, int l=0, int r=-1) {
    if(r < 0) r = n;

    eval(k, l, r);

    if(b <= l || r <= a) return;
    
    if(a <= l && r <= b) {
        lazy[k] *= x;
        eval(k, l, r);
    }

    else {
        add(a, b, x, 2*k+1, l, (l+r)/2);
        add(a, b, x, 2*k+2, (l+r)/2, r);
        node[k] = node[2*k+1] + node[2*k+2];
    }
  }
  ll getsum(int a, int b, int k=0, int l=0, int r=-1) {
    if(r < 0) r = n;

    eval(k, l, r);

    if(b <= l || r <= a) return 0;
    if(a <= l && r <= b) return node[k];
    ll vl = getsum(a, b, 2*k+1, l, (l+r)/2);
    ll vr = getsum(a, b, 2*k+2, (l+r)/2, r);
    return vl + vr;
  }

};


vector<string> split_string(string);

// Complete the xorQueries function below.
vector<long> xorQueries(vector<int> a, int m, int p) {
  // Return an array consisting of the answers of all type-2 queries.
  const int B = 17;
  vector<LazySegmentTree> st(B);
  int n = a.size();
  REP(b, 0, B) {
    VI acc(n + 1);
    REP(i, 0, n) {
      acc[i + 1] = acc[i] ^ ((a[i] >> b) & 1);
    }
    vector<int> aux(n);
    REP(i, 0, n - p + 1) {
      aux[i] = 2 * (acc[i + p] ^ acc[i]) - 1;
    }
    st[b].init(aux);
  }
  vector<long> ret;
  REP(i, 0, m) {
    int ty, l, r;
    cin >> ty >> l >> r;
    if (ty == 1) {
      l--;
      int lo = max(0, l - p + 1);
      int hi = min(l, n - p) + 1;
      if (lo < hi) {
	REP(b, 0, B) {
	  if (r & 1 << b) {
	    st[b].add(lo, hi, -1);
	  }
	}
      }
    } else {
      ll ans = 0;
      l--;
      int hi = min(r, n - p + 1);
      if (l < hi) {
	REP(b, 0, B) {
	  ll tap = st[b].getsum(l, hi);
	  tap = (tap + hi - l) / 2;
	  ans += tap << b;
	}
      }
      ret.push_back(ans);
    }
    if (DEBUG) {
      REP(b, 0, 4) {
	REP(i, 0, n - p + 1) {
	  cerr << " " << st[b].getsum(i, i + 1);
	}
	cerr << endl;
      }
      cerr << "=========" << endl;
    }
  }
  return ret;
}

int main()
{
    ofstream fout(getenv("OUTPUT_PATH"));

    string nmp_temp;
    getline(cin, nmp_temp);

    vector<string> nmp = split_string(nmp_temp);

    int n = stoi(nmp[0]);

    int m = stoi(nmp[1]);

    int p = stoi(nmp[2]);

    string a_temp_temp;
    getline(cin, a_temp_temp);

    vector<string> a_temp = split_string(a_temp_temp);

    vector<int> a(n);

    for (int i = 0; i < n; i++) {
        int a_item = stoi(a_temp[i]);

        a[i] = a_item;
    }

    vector<long> result = xorQueries(a, m, p);

    for (int i = 0; i < result.size(); i++) {
        fout << result[i];

        if (i != result.size() - 1) {
            fout << "\n";
        }
    }

    fout << "\n";

    fout.close();

    return 0;
}

vector<string> split_string(string input_string) {
    string::iterator new_end = unique(input_string.begin(), input_string.end(), [] (const char &x, const char &y) {
        return x == y and x == ' ';
    });

    input_string.erase(new_end, input_string.end());

    while (input_string[input_string.length() - 1] == ' ') {
        input_string.pop_back();
    }

    vector<string> splits;
    char delimiter = ' ';

    size_t i = 0;
    size_t pos = input_string.find(delimiter);

    while (pos != string::npos) {
        splits.push_back(input_string.substr(i, pos - i));

        i = pos + 1;
        pos = input_string.find(delimiter, i);
    }

    splits.push_back(input_string.substr(i, min(pos, input_string.length()) - i + 1));

    return splits;
}
