/**
 * https://beta.atcoder.jp/contests/bitflyer2018-final/submissions/2762276
 */

string s;
int len;
VI tbl;

int n;
ModInt<> a[200000];
ModInt<> coef_gl[200000];

struct node {
  ModInt<> coef, val;
  char ty;
  int idx;
  node *ch[2];
  node(void) {
    ty = '&';
    coef = 1;
    val = 0;
    REP(i, 0, 2) ch[i] = 0;
    idx = -1;
  }
  node(char c): coef(1), val(0), ty(c) {
    REP(i, 0, 2) ch[i] = 0;
    idx = -1;
  }
  void debug(int lv = 0) const {
    REP(i, 0, lv)cerr<<" ";
    cerr << ty << "[" << val << "](" << coef << ")" << endl;
    REP(i, 0, 2) {
      if (ch[i] != NULL) {
        ch[i]->debug(lv + 1);
      }
    }
  }
  void calc(void) {
    if (ty == 'a') {
      assert (idx >= 0);
      val = a[idx];
      return;
    }
    REP(i, 0, 2) ch[i]->calc();
    if (ty == '+') {
      val = ch[0]->val + ch[1]->val;
      return;
    }
    if (ty == '-') {
      val = ch[0]->val - ch[1]->val;
      return;
    }
    assert (ty == '*');
    val = ch[0]->val * ch[1]->val;
    return;
  }
  void dfs(ModInt<> cc) {
    if (ty == 'a') {
      coef = cc;
      coef_gl[idx] = cc;
      return;
    }
    if (ty == '+') {
      coef = cc;
      REP(i, 0, 2) ch[i]->dfs(cc);
      return;
    }
    if (ty == '-') {
      coef = cc;
      ch[0]->dfs(cc);
      ch[1]->dfs(ModInt<>(0)-cc);
      return;
    }
    assert (ty == '*');
    ch[0]->dfs(cc*ch[1]->val);
    ch[1]->dfs(cc*ch[0]->val);
    return;
  }
};

int pos = 0;

node* parse_term(void);
node* parse_value(void);
node* parse_expr(void) {
  if (pos == len) {
    assert(!"empty");
  }
  node* res = parse_term();
  while (pos < len && (s[pos] == '+' || s[pos] == '-')) {
    char nxt = s[pos];
    pos++;
    node* sub2 = parse_term();
    node *ret = new node(nxt);
    ret->ch[1] = sub2;
    ret->ch[0] = res;
    res = ret;
  }
  return res;
}
node* parse_term(void) {
  if (pos == len) {
    assert(!"empty");
  }
  node* res = parse_value();
  while (pos < len && s[pos] == '*') {
    pos++;
    node* sub2 = parse_value();
    node *ret = new node('*');
    ret->ch[1] = sub2;
    ret->ch[0] = res;
    res = ret;
  }
  return res;
}
node* parse_value(void) {
  if (pos == len) {
    assert(!"empty");
  }
  char st = s[pos];
  if (st == '(') {
    pos++;
    node* res = parse_expr();
    pos++;
    return res;
  }
  assert (st == 'a');
  node *ret = new node('a');
  ret->idx = tbl[pos];
  pos++;
  return ret;
}

