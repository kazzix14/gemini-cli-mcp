# Gemini CLI MCP 使い方ガイド

## セットアップ完了！

ビルドが完了し、実行可能ファイルが以下に作成されました：
```
/Users/kazuma/dev/gemini-cli-mcp/target/release/gemini-cli-mcp
```

## Claude Codeでの使い方

### 1. 設定確認
Claude Codeの設定（settings.json）に以下が追加されていることを確認：

```json
{
  "mcpServers": {
    "gemini-cli": {
      "command": "/Users/kazuma/dev/gemini-cli-mcp/target/release/gemini-cli-mcp"
    }
  }
}
```

### 2. 使用例

#### シンプルな質問
```
Geminiに聞いて：TypeScriptの型ガードについて説明して
```

#### ファイルを参照した分析
```
Geminiでsrc/main.rsのコードをレビューして改善点を教えて
```

#### 複数ファイルの比較
```
GeminiでCargo.tomlとCargo.lockを見て、依存関係の構造を説明して
```

#### コードリファクタリング
```
Geminiを使ってtest.jsのコードをTypeScriptに変換して
```

### 3. 高度な使い方

#### モデルの指定
デフォルトは`gemini-2.5-pro`ですが、`gemini-2.5-flash`も使えます：

```
Geminiのgemini-2.5-flashモデルを使って、package.jsonの依存関係を最適化して
```

## トラブルシューティング

### MCPサーバーが動作しない場合
1. Claude Codeを再起動
2. .envファイルにGOOGLE_CLOUD_PROJECTが設定されているか確認
3. Gemini CLIが正しくインストールされているか確認（`which gemini`）

### エラーが出る場合
- Gemini CLIの認証が正しく設定されているか確認
- プロジェクトIDが正しいか確認

## 今後の改善案

- ストリーミング対応
- より多くのGemini CLIオプションのサポート
- エラーハンドリングの改善