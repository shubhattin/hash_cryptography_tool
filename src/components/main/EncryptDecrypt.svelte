<form>
  <div class="grid">
    <label>
      <input type="radio" name="option" value="encrypt" /> Encrypt
    </label>
    <label>
      <input type="radio" name="option" value="decrypt" /> Decrypt
    </label>
  </div>
  <label> Enter Text <textarea name="text" required></textarea> </label>
  <label> Enter Key <input type="password" name="key" required /> </label>
  <button type="submit">Sumbit</button>
  <label>
    Output Text <span class="clear_btn"></span>
    <span class="copy_btn"></span>
    <span class="spinner htmx-indicator" id="encrypt_decrypt_spinner"></span>
    <div id="encrypt_decrypt_out">
      <textarea readonly></textarea>
    </div>
  </label>
</form>
